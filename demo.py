from wasmtime import Engine, Store, Config
import requests
from pybind.imports import (
    creds_client,
    http_client,
    time_client,
    print_client,
    connector_types,
)

# from pybind.exports import connector_types
import os
import time
import pybind


class CredsClient(creds_client.CredsClient):
    # TODO: use actual lang SDK for credential resolution
    def get_creds(self) -> creds_client.Creds:
        access_key_id = os.environ["AWS_ACCESS_KEY_ID"]
        secret_access_key = os.environ["AWS_SECRET_ACCESS_KEY"]
        session_token = os.environ["AWS_SESSION_TOKEN"]

        if access_key_id == None or secret_access_key == None or session_token == None:
            raise RuntimeError("Credentials not correctly configured")

        return creds_client.Creds(access_key_id, secret_access_key, session_token)


class HostHTTPClient(http_client.HttpClient):
    def make_http_request(
        self, options: http_client.HttpCallOptions
    ) -> http_client.HttpReturnValues:
        headers_dict = {}
        for k, v in options.headers:
            headers_dict[k] = v

        print(f"In python method making request to: {options.uri}")
        response = requests.request(
            options.method.name, options.uri, data=options.body, headers=headers_dict
        )
        return http_client.HttpReturnValues(response.status_code, response.content)


class PrintClient(print_client.PrintClient):
    def print_host(self, input: str) -> None:
        print(input)


class TimeClient(time_client.TimeClient):
    def get_sys_time_unix_millis(self) -> int:
        now_ms = int(time.time_ns() / 1000000)
        return now_ms


class ConnectorTypes(connector_types.ConnectorTypes):
    pass


def main():
    config = Config()
    engine = Engine(config)
    store = Store(engine)
    host_imports = pybind.RootImports(
        http_client=HostHTTPClient(),
        print_client=PrintClient(),
        time_client=TimeClient(),
        creds_client=CredsClient(),
        connector_types=ConnectorTypes(),
    )
    root = pybind.Root(store, host_imports)
    res = root.list_tables(store)
    print("Output in Python: \n", res.value)


if __name__ == "__main__":
    main()
