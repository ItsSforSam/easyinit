from __future__ import annotations
from warnings import warn
import typing as t
import logging as l
import sys
import os

__all__ = ["get_logger","ColorFormatter","debug","info","warning","error"]
COLOR_FORCE:bool = True if os.getenv("FORCE_COLOR") in ("1","true","True","TRUE") else False


class ColorFormatter(l.Formatter):
    COLOR_CODES = {
        "DEBUG": "\x1b[2;37m",  # Dimmed white
        "INFO": "\x1b[32m",     # Green
        "WARNING": "\x1b[33m",  # Yellow
        "ERROR": "\x1b[31m",    # Red
        "CRITICAL": "\x1b[1;31m", # Bold Red, used in exceptions
    }
    RESET_CODE = "\x1b[0m"

    def format(self, record: l.LogRecord) -> str:
        color_code = self.COLOR_CODES.get(record.levelname, self.RESET_CODE)
        record.msg = f"{color_code}{record.msg}{self.RESET_CODE}"
        return super().format(record)

# def init_logger():
#     # l.basicConfig
#     l.basicConfig(
        
#     )

def get_logger(name: str) -> l.Logger:
    logger = l.getLogger(name)
    handler = l.StreamHandler(sys.stdout)
    if COLOR_FORCE or (sys.stdout.isatty() and sys.stderr.isatty()):
        formatter = ColorFormatter("%(asctime)s - %(name)s - %(levelname)s - %(message)s")
    else:
        formatter = l.Formatter("%(asctime)s - %(name)s - %(levelname)s - %(message)s")
    handler.setFormatter(formatter)
    logger.addHandler(handler)
    return logger

# re-export logging functions
debug = l.debug
info = l.info
warning = l.warning
error = l.error
