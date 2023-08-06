from abc import abstractmethod
from dataclasses import dataclass
from enum import Enum
from typing import List, Protocol, Tuple

class Methods(Enum):
    GET = 0
    POST = 1
    PUT = 2
    DELETE = 3
    OPTIONS = 4
    HEAD = 5

Headers = List[Tuple[str, str]]
@dataclass
class HttpCallOptions:
    method: Methods
    uri: str
    headers: Headers
    body: bytes

@dataclass
class HttpReturnValues:
    status: int
    body: bytes

class HttpClient(Protocol):
    @abstractmethod
    def make_http_request(self, options: HttpCallOptions) -> HttpReturnValues:
        raise NotImplementedError

