import logging
from timeit import default_timer
from typing import Callable

log = logging.getLogger(__name__)


class FPSCounter:
    def __init__(self, timer: Callable = default_timer, log_every_s: float = 1.0):
        self.timer = timer
        self.log_every_s = log_every_s

        self._clock_checkpoint: float = 0.0
        self._frames_passed: int = 0

    def start(self):
        self._frames_passed = 0
        self._clock_checkpoint = self.timer()

    def frame_done(self):
        self._frames_passed += 1
        now = self.timer()
        time_passed = now - self._clock_checkpoint

        if time_passed >= self.log_every_s:
            fps = self._frames_passed / time_passed
            log.info("%d FPS", fps)

            self._clock_checkpoint = now
            self._frames_passed = 0
