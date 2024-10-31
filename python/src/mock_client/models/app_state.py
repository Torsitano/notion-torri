from enum import Enum


class AppState(str, Enum):
    CLOSED = "Closed"
    DISCOVERED = "Discovered"
    SANCTIONED = "Sanctioned"

    def __str__(self) -> str:
        return str(self.value)
