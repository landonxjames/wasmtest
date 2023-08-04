from ..imports import connector_types
from abc import abstractmethod
from typing import Protocol

HttpCallOptions = connector_types.HttpCallOptions
HttpReturnValues = connector_types.HttpReturnValues
class HttpClient(Protocol):
    @abstractmethod
    def make_http_request(self, options: HttpCallOptions) -> HttpReturnValues:
        raise NotImplementedError

