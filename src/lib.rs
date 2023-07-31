use async_trait::async_trait;
use aws_credential_types::{cache::CredentialsCache, provider::ProvideCredentials, Credentials};
use aws_sdk_dynamodb::{config::Region, Client};
use aws_smithy_async::rt::sleep::{AsyncSleep, SharedAsyncSleep, Sleep};
use aws_smithy_async::time::{SharedTimeSource, TimeSource};
use aws_smithy_client::erase::DynConnector;
use aws_smithy_http::{body::SdkBody, result::ConnectorError};
use fluvio_wasm_timer;
use std::time::Duration;
use wasm_bindgen::{prelude::*, JsCast};

extern crate console_error_panic_hook;

//TODO: replace this with something that will log from WASI runtimes?
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub async fn list_tables() -> Result<String, String> {
    console_error_panic_hook::set_once();

    let credentials_provider = static_credential_provider();
    let credentials = credentials_provider.provide_credentials().await.unwrap();
    let access_key = credentials.access_key_id();

    let shared_config = aws_config::from_env()
        .time_source(WasmTimeSource)
        .sleep_impl(WasmSleep)
        .region(Region::new("us-east-1"))
        .credentials_provider(credentials_provider)
        .credentials_cache(wasm_credentials_cache())
        .http_connector(DynConnector::new(Adapter::new(true)))
        .load()
        .await;
    let ddb_client = Client::new(&shared_config);

    let now = WasmTimeSource
        .now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::new(0, 0));
    log!("current date in unix timestamp: {}", now.as_secs());

    let resp = ddb_client
        .list_tables()
        .send()
        .await
        .map_err(|e| format!("{:?}", e))?;
    log!("Panics before here");

    let tables = resp.table_names().unwrap_or_default();

    for table in tables {
        log!("Table Name: {}", table);
    }
    let output = tables.iter().fold("".to_string(), |mut acc, name| {
        acc.push_str(name);
        acc.push_str("\n");
        acc
    });
    log!("{output}");
    Ok(output)
}

#[derive(Debug)]
struct WasmTimeSource;
impl TimeSource for WasmTimeSource {
    fn now(&self) -> std::time::SystemTime {
        let wasm_now = fluvio_wasm_timer::SystemTime::now()
            .duration_since(fluvio_wasm_timer::SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0));
        let sys_now = std::time::SystemTime::UNIX_EPOCH
            .checked_add(wasm_now)
            .unwrap();
        sys_now
    }
}

#[derive(Debug, Clone)]
struct WasmSleep;
impl AsyncSleep for WasmSleep {
    fn sleep(&self, duration: std::time::Duration) -> Sleep {
        Sleep::new(Box::pin(async move {
            fluvio_wasm_timer::Delay::new(duration).await.unwrap();
        }))
    }
}

fn static_credential_provider() -> impl ProvideCredentials {
    // let credentials = serde_wasm_bindgen::from_value::<AwsCredentials>(retrieve_credentials())
    //     .expect("invalid credentials");
    Credentials::from_keys("fake", "fake", Some("fake".to_string()))
}

fn wasm_credentials_cache() -> CredentialsCache {
    let shared_sleep = SharedAsyncSleep::new(WasmSleep);
    let shared_time = SharedTimeSource::new(WasmTimeSource);
    CredentialsCache::lazy_builder()
        .sleep(shared_sleep)
        .time_source(shared_time)
        .into_credentials_cache()
}

#[derive(Debug, Clone)]
struct Adapter {
    verbose: bool,
}

impl Adapter {
    fn new(verbose: bool) -> Self {
        Self { verbose }
    }
}

impl tower::Service<http::Request<SdkBody>> for Adapter {
    type Response = http::Response<SdkBody>;

    type Error = ConnectorError;

    #[allow(clippy::type_complexity)]
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>,
    >;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: http::Request<SdkBody>) -> Self::Future {
        let (parts, body) = req.into_parts();

        let uri = parts.uri.to_string();
        if self.verbose {
            log!("sending request to {}", uri);
            log!("http::Request parts: {:?}", parts);
            log!("http::Request body: {:?}", body);
            log!("");
        }

        let (tx, rx) = tokio::sync::oneshot::channel();

        log!("begin request...");
        wasm_bindgen_futures::spawn_local(async move {
            let fut = WasmHttpClient::send(parts, body);

            // let blah = Box::pin(
            //     reqwest::Client::new()
            //         .request(req.method().clone(), req.uri().clone().to_string())
            //         .body(body.bytes().unwrap())
            //         .headers(req.headers().clone())
            //         .send(),
            // );

            let _ = tx.send(fut.await.unwrap_or_else(|val| {
                panic!("failure while making request to: {} \n {:#?}", uri, val)
            }));
        });

        Box::pin(async move {
            let response = rx.await.map_err(|e| ConnectorError::user(Box::new(e)))?;
            log!("response received");
            Ok(response)
        })
    }
}

//TODO: Figure out how to do this in a WASI runtime
/// At this moment, there is no standard mechanism to make an outbound
/// HTTP request from within the guest Wasm module.
/// Eventually that will be defined by the WebAssembly System Interface:
/// https://github.com/WebAssembly/wasi-http
#[async_trait(?Send)]
trait MakeRequestWasm {
    async fn send(
        parts: http::request::Parts,
        body: SdkBody,
    ) -> Result<http::Response<SdkBody>, JsValue>;
}

pub struct WasmHttpClient;

//TODO This might need a node-fetch shim?
#[async_trait(?Send)]
impl MakeRequestWasm for WasmHttpClient {
    /// The [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API)
    /// will be used to actually send the outbound HTTP request.
    /// Most of the logic here is around converting from
    /// the [http::Request]'s shape to [web_sys::Request].
    async fn send(
        parts: http::request::Parts,
        body: SdkBody,
    ) -> Result<http::Response<SdkBody>, JsValue> {
        // use js_sys::{Array, ArrayBuffer, Reflect, Uint8Array};
        // use wasm_bindgen_futures::JsFuture;

        // let mut opts = web_sys::RequestInit::new();
        // opts.method(parts.method.as_str());
        // opts.mode(web_sys::RequestMode::Cors);

        // let body_pinned = std::pin::Pin::new(body.bytes().unwrap());
        // if body_pinned.len() > 0 {
        //     let uint_8_array = unsafe { Uint8Array::view(&body_pinned) };
        //     opts.body(Some(&uint_8_array));
        // }

        // let request = web_sys::Request::new_with_str_and_init(&parts.uri.to_string(), &opts)?;

        // for (name, value) in parts
        //     .headers
        //     .iter()
        //     .map(|(n, v)| (n.as_str(), v.to_str().unwrap()))
        // {
        //     request.headers().set(name, value)?;
        // }

        // let window = web_sys::window().ok_or("could not get window")?;
        // let promise = window.fetch_with_request(&request);
        // let res_web = JsFuture::from(promise).await?;
        // let res_web: web_sys::Response = res_web.dyn_into().unwrap();

        // let promise_array = res_web.array_buffer()?;
        // let array = JsFuture::from(promise_array).await?;
        // let buf: ArrayBuffer = array.dyn_into().unwrap();
        // let slice = Uint8Array::new(&buf);
        // let body = slice.to_vec();

        // let mut builder = http::Response::builder().status(res_web.status());
        // for i in js_sys::try_iter(&res_web.headers())?.unwrap() {
        //     let array: Array = i?.into();
        //     let values = array.values();

        //     let prop = String::from("value").into();
        //     let key = Reflect::get(values.next()?.as_ref(), &prop)?
        //         .as_string()
        //         .unwrap();
        //     let value = Reflect::get(values.next()?.as_ref(), &prop)?
        //         .as_string()
        //         .unwrap();
        //     builder = builder.header(&key, &value);
        // }
        // let res_body = SdkBody::from(body);
        // let res = builder.body(res_body).unwrap();
        // Ok(res)
        let body_bytes = body.bytes().unwrap().to_vec();

        let blah = reqwest::Client::new()
            .request(parts.method, parts.uri.to_string())
            .body(body_bytes)
            .headers(parts.headers)
            .send()
            .await
            .unwrap_or_else(|err| panic!("failure while making request: {}", err));
        let mut builder = http::Response::builder().status(blah.status());
        let blah_body = SdkBody::from(blah.bytes().await.unwrap());

        let blah_res = builder.body(blah_body).unwrap();

        Ok(blah_res)
    }
}
