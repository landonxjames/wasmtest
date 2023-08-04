from abc import abstractmethod
from typing import Protocol

class PrintClient(Protocol):
    @abstractmethod
    def print_host(self, input: str) -> None:
        raise NotImplementedError

