from .connector_types import ConnectorTypes
from .creds_client import CredsClient
from .http_client import HttpClient
from .print_client import PrintClient
from .time_client import TimeClient
from dataclasses import dataclass

@dataclass
class RootImports:
    connector_types: ConnectorTypes
    http_client: HttpClient
    time_client: TimeClient
    print_client: PrintClient
    creds_client: CredsClient
