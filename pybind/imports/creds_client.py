from abc import abstractmethod
from dataclasses import dataclass
from typing import Protocol

@dataclass
class Creds:
    access_key_id: str
    secret_access_key: str
    session_token: str

class CredsClient(Protocol):
    @abstractmethod
    def get_creds(self) -> Creds:
        raise NotImplementedError

