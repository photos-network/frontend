import enum


class FrontendState(enum.Enum):
    """Represent the current state of Photos.network frontend."""

    not_running = "NOT_RUNNING"
    starting = "STARTING"
    running = "RUNNING"
    stopping = "STOPPING"
    final_write = "FINAL_WRITE"
    stopped = "STOPPED"

    def __str__(self) -> str:  # pylint: disable=invalid-str-returned
        """Return the event."""
        return self.value  # type: ignore
