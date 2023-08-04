from dataclasses import dataclass
from enum import Enum
from typing import List, Tuple

from typing import TYPE_CHECKING
if TYPE_CHECKING:
  from .. import Root

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

class ConnectorTypes:
    component: 'Root'
    
    def __init__(self, component: 'Root') -> None:
        self.component = component
