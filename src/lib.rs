//WIT imports
wit_bindgen::generate!("utils");
use act::utils::connector_types::{HttpCallOptions, Methods};
use act::utils::{creds_client, http_client, print_client, time_client};
//Normal crate imports
use aws_credential_types::{provider::ProvideCredentials, Credentials};
use aws_sdk_dynamodb::{config::Region, Client};
use aws_smithy_async::rt::sleep::{AsyncSleep, Sleep};
use aws_smithy_async::time::TimeSource;
use aws_smithy_client::erase::DynConnector;
use aws_smithy_http::{
    body::{Error, SdkBody},
    result::ConnectorError,
};
use std::str::FromStr;
use std::time::Duration;

macro_rules! log {
    ( $( $t:tt )* ) => {
        act::utils::print_client::print_host(&format!( $( $t )* ));
    }
}

impl FromStr for Methods {
    //TODO: make this a real err
    type Err = ();

    fn from_str(input: &str) -> Result<Methods, Self::Err> {
        match input {
            "OPTIONS" => Ok(Methods::Options),
            "GET" => Ok(Methods::Get),
            "HEAD" => Ok(Methods::Head),
            "PUT" => Ok(Methods::Put),
            "POST" => Ok(Methods::Post),
            "DELETE" => Ok(Methods::Delete),
            _ => Err(()),
        }
    }
}

struct ActUtils;

impl Utils for ActUtils {
    fn list_tables() -> Result<String, String> {
        //Spawning tokio runtime to run the async call in a sync context
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap_or_else(|err| {
                log!("{}", err);
                panic!("FAILED TO GEN RUNTIME")
            });
        let res = rt.block_on(list_tables());
        res
    }
}

//Exporting the functions defined in WIT to WASM
export_utils!(ActUtils);

pub async fn list_tables() -> Result<String, String> {
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
    Ok(output)
}

#[derive(Debug)]
struct WasmTimeSource;
impl TimeSource for WasmTimeSource {
    fn now(&self) -> std::time::SystemTime {
        let host_now = act::utils::time_client::get_sys_time_unix_millis();
        let dur = Duration::from_millis(host_now);
        let sys_now = std::time::SystemTime::UNIX_EPOCH.checked_add(dur).unwrap();
        sys_now
    }
}

#[derive(Debug, Clone)]
struct WasmSleep;
impl AsyncSleep for WasmSleep {
    fn sleep(&self, duration: std::time::Duration) -> Sleep {
        Sleep::new(Box::pin(async move {
            tokio::time::sleep(duration).await;
        }))
    }
}

//TODO: figure out how to pass the Lambda env variable credentials to the WASM code
//TODO: expose a WIT imported function that provides the credentials
fn static_credential_provider() -> impl ProvideCredentials {
    let creds = creds_client::get_creds();
    Credentials::from_keys(
        creds.access_key_id,
        creds.secret_access_key,
        Some(creds.session_token),
    )
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

        log!("begin request...");
        let res = WasmHttpClient::send(parts, body).map_err(|e| ConnectorError::user(e));
        Box::pin(async move { res })
    }
}

trait MakeRequestWasm {
    fn send(parts: http::request::Parts, body: SdkBody) -> Result<http::Response<SdkBody>, Error>;
}

pub struct WasmHttpClient;

impl MakeRequestWasm for WasmHttpClient {
    fn send(parts: http::request::Parts, body: SdkBody) -> Result<http::Response<SdkBody>, Error> {
        let body_bytes = body.bytes().unwrap().to_vec();

        let headers_vec: Vec<(String, String)> = parts
            .headers
            .into_iter()
            .map(|(key, val)| (key.unwrap().to_string(), val.to_str().unwrap().to_string()))
            .collect();

        let http_opts = &HttpCallOptions {
            method: Methods::from_str(parts.method.as_str()).unwrap(),
            uri: parts.uri.to_string(),
            headers: headers_vec,
            body: body_bytes,
        };

        log!(
            "Calling the host language http_client with {:#?}",
            http_opts
        );
        let res = http_client::make_http_request(http_opts);
        log!("returned to rust from host http call");
        let builder = http::Response::builder().status(res.status);
        let sdk_body = SdkBody::from(res.body);
        let sdk_res = builder.body(sdk_body)?;

        Ok(sdk_res)
    }
}
