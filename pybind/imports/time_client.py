from abc import abstractmethod
from typing import Protocol

class TimeClient(Protocol):
    @abstractmethod
    def get_sys_time_unix_millis(self) -> int:
        raise NotImplementedError

