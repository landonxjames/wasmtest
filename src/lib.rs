use async_trait::async_trait;
use aws_credential_types::{provider::ProvideCredentials, Credentials};
use aws_sdk_dynamodb::{config::Region, Client};
use aws_smithy_async::rt::sleep::{AsyncSleep, Sleep};
use aws_smithy_async::time::TimeSource;
use aws_smithy_client::erase::DynConnector;
use aws_smithy_http::{body::SdkBody, result::ConnectorError};
use fluvio_wasm_timer;
use std::time::Duration;
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

//TODO: replace this with something that will log from non JS hosts envs?
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub async fn list_tables() -> Result<String, String> {
    console_error_panic_hook::set_once();

    let credentials_provider = static_credential_provider();

    let shared_config = aws_config::from_env()
        .time_source(WasmTimeSource)
        .sleep_impl(WasmSleep)
        .region(Region::new("us-east-1"))
        .credentials_provider(credentials_provider)
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

//TODO: figure out how to pass the Lambda env variable credentials to the WASM code
fn static_credential_provider() -> impl ProvideCredentials {
    // let credentials = serde_wasm_bindgen::from_value::<AwsCredentials>(retrieve_credentials())
    //     .expect("invalid credentials");
    Credentials::from_keys("fake", "fake", Some("fake".to_string()))
}

//Code below here is all dedicated to sending the call across the sandbox barrier
//to be executed by the host runtime
#[derive(Debug, Clone)]
struct Adapter {
    verbose: bool,
}

impl Adapter {
    fn new(verbose: bool) -> Self {
        Self { verbose }
    }
}

//Replacing the default call impl for the SDK
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

        let (send_channel, rec_channel) = tokio::sync::oneshot::channel();

        log!("begin request...");
        wasm_bindgen_futures::spawn_local(async move {
            let fut = WasmHttpClient::send(parts, body);
            let _ = send_channel.send(fut.await.unwrap_or_else(|val| {
                panic!("failure while making request to: {} \n {:#?}", uri, val)
            }));
        });

        Box::pin(async move {
            let response = rec_channel
                .await
                .map_err(|e| ConnectorError::user(Box::new(e)))?;
            log!("response received");
            Ok(response)
        })
    }
}

#[async_trait(?Send)]
trait MakeRequestWasm {
    async fn send(
        parts: http::request::Parts,
        body: SdkBody,
    ) -> Result<http::Response<SdkBody>, JsValue>;
}

pub struct WasmHttpClient;

#[async_trait(?Send)]
impl MakeRequestWasm for WasmHttpClient {
    async fn send(
        parts: http::request::Parts,
        body: SdkBody,
    ) -> Result<http::Response<SdkBody>, JsValue> {
        let body_bytes = body.bytes().unwrap().to_vec();
        //Reqwest uses wasm-bindgen under the hood to bind calls to fetch
        //https://github.com/seanmonstar/reqwest/blob/61b1b2b5e6dace3733cdba291801378dd974386a/src/wasm/client.rs#L12
        let res = reqwest::Client::new()
            .request(parts.method, parts.uri.to_string())
            .body(body_bytes)
            .headers(parts.headers)
            .send()
            .await
            .unwrap_or_else(|err| panic!("failure while making request: {}", err));

        let builder = http::Response::builder().status(res.status());
        let sdk_body = SdkBody::from(res.bytes().await.unwrap());
        let sdk_res = builder.body(sdk_body).unwrap();

        Ok(sdk_res)
    }
}
