#!/bin/env python3
from __future__ import annotations
from warnings import warn
import typing as t
import os
from pathlib import Path
import sys
import tomllib
import argparse
import importlib
from functools import wraps
logging = importlib.import_module("tools.py-logging") # allows importing from tools/py-logging


__version__ = "0.0.1"

if t.TYPE_CHECKING:
    T = t.TypeVar("T")

ROOT = Path(os.path.dirname(__file__))
# CSpell: words BUILDABLES
BUILDABLES = [ "easyinit", "easyctl", "journald"]
"""
Packages that are provided
"""

UNSTABLE_FEATURES = ["selinux","systemd"]
"""
Provides a warning that unstable features are being used.
"""

l = logging.get_logger("configure")

def main():
    parsed, conf =  get_parser().parse_known_args()
    conf = parse_conf(conf)
    if parsed.verbose:
        l.setLevel(logging.l.DEBUG)
    generate_makefile(conf, parsed)
    
def noprame_cache(f:t.Callable[[],T])->t.Callable[[],T]:
    """
    Used for caching where it does not take any parameters

    Reason to use this:
    When a function may be called multiple times and can be expensive to calculate.
    Like, calling a external program to retrieve the host triple.

    """
    _sent = object()
    cached = _sent
    @wraps(f)
    def wrapper() -> T:
        nonlocal _sent, cached
        if cached is _sent:
            cached = f()
        return cached
    return wrapper
def get_version() -> str:
    with open(ROOT / "Cargo.toml", "rb") as f:
        data = tomllib.load(f)
    return data["workspace"]["package"]["version"]

def eprint(*args, **kwargs):
    file = kwargs.pop("file", sys.stderr)
    print(*args, file=sys.stderr, **kwargs)
def get_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Configure EasyInit before building."
    )
    parser.add_argument(
        "--version", "-V",
        action="version",
        version=f"Easyinit {get_version()}\nConfigure Script: {__version__}",
        help="Show the version number of Easyinit and this configure script.",
    )
    parser.add_argument(
        "--prefix",
        help="Set the installation prefix (default: %(default)s).",
        default=Path("/usr/local"),
        type=Path,
    )
    parser.add_argument(
        "--selinux",
        action=OptionalFeature,
        help="Enable or disable SELinux support.",
        default=False,
    )
    parser.add_argument(
        "--coreutils",
        action=OptionalFeature,
        help="Enable or disable Coreutils integration.",
        )
    parser.add_argument(
        "-v", "--verbose",
        action="store_true",
        help="Make this script more talkative",
    )
    parser.add_argument(
        "--output", "-o",
        help="Where to put the generated Makefile. Default is in the root of the repo",
        default=ROOT / "Makefile",
        type=Path,
        metavar="PATH"
    )
    parser.add_argument(
    "--systemd",
    help="Allow systemd integration. Default is when systemd is detected on the system. (Detected: %(default)s)",
    default=is_systemd(),
    action=OptionalFeature,
    )
    parser.add_argument(
        "--frozen", "--offline",
        help="Build without accessing the network. Passes --frozen to cargo. Recommend to pass `prefetch` target before compiling",
        action="store_true",
        dest="frozen",
    )
    parser.add_argument(
        "--target",
        help="Set the compilation target triple. Default is the host system's triple.",
        type=str,
        metavar="TARGET-TRIPLE"
    )
    parser.add_argument(
        "--debug", "--no-release",
        help="Disable release mode on compiling flags",
        action="store_true",
        dest="debug"
    )
    parser.add_argument(
        "--native",
        action="store_true",
        help="Compiles optimizing for the current cpu. This "
    )
    return parser

@noprame_cache
def get_host_triple() -> str:
    l.debug("Determining host triple using rustc")
    import subprocess
    
    result = subprocess.run(["rustc", "-vV"], capture_output=True, text=True, encoding="UTF-8", check=True)
    l.debug(f"rustc result:\n{result}")
    for line in result.stdout.splitlines():
        if line.startswith("host:"):
            host = line.split(":", 1)[1].strip()
            l.debug(f"Host determined to be `{host}`")
            return host
    raise RuntimeError("Could not determine host triple from rustc output.")

def is_systemd() -> bool:
    """
    Check if systemd is available on this system
    
    If systemctl is in PATH, we assume systemd is available.
    This is to prevent leaving a system stranded without proper service files and to make an 
    easier transition.

    """
    import shutil
    return shutil.which("systemctl") is not None

class OptionalFeature(argparse.Action):
    """
    Handles the --enable-<feature>, --disable-<feature> options
    """

    def __init__(self,
                 option_strings, 
                 dest,
                 nargs = None,
                 const = None,
                 default = None,
                 type = None,
                 choices = None,
                 required = False,
                 help = None,
                 metavar = None,
                 deprecated = False
    ) -> None:
        self.feature_name = option_strings[0].lstrip("-")
        _option_strs = []
        # ops:str
        for ops in option_strings:
            _option_strs.append(ops)
            ops = ops.lstrip("-")
            _option_strs.append("--enable-" + ops)
            _option_strs.append("--with-" + ops)
            _option_strs.append("--disable-" + ops)
            _option_strs.append("--without-" + ops)
        
        
        
        super().__init__(option_strings=_option_strs,
                         dest=dest,
                         nargs=0,
                         const=const,
                         default=default,
                         type=type,
                         choices=choices,
                         required=required,
                         help=help,
                         metavar=metavar,
                         deprecated=deprecated
                         )
    # def __init__(self, option_strings, dest, **kwargs):
    #     super().__init__(option_strings, dest, nargs=0, **kwargs)

    def __call__(self, parser, namespace, values, option_string = None):
        if option_string in self.option_strings:
            if option_string.startswith("--enable-") or option_string.startswith("--with-"):
                l.warn(f"Enabling unstable {self.feature_name} feature")
                setattr(namespace, self.dest, True)
            elif option_string.startswith("--disable-") or option_string.startswith("--without-"):
                setattr(namespace, self.dest, False)
            elif callable(self.default): # allows lazy evaluation of defaults
                setattr(namespace, self.dest, self.default())
            else:
                setattr(namespace, self.dest, self.default)
    
    def format_usage(self) -> str:
        
        return " | ".join(self.option_strings)
    
    
    # def __call__(self, parser, namespace, values, option_string=None):
    #     features = getattr(namespace, self.dest, set())
    #     feature_name = option_string.lstrip("-")
    #     features.add(feature_name)
    #     setattr(namespace, self.dest, features)

class Conf:
    rustc: str
    cargo: str
    additional_options: t.Optional[dict[str,str]]
    def __init__(self,rustc:str="rustc",cargo:str="cargo",**kwargs) -> None:
        self.rustc = rustc
        self.cargo = cargo
        self.additional_options = None
        if len(kwargs) != 0:
            self.additional_options = kwargs
            for k in kwargs.keys():

                warn(f"Unknown variable `{k}`, ignoring", stacklevel=1)

def parse_conf(conf:list[str])->Conf:
    options = {}
    for x in conf:
        a = x.split("=",1)
        options[a[0].lower()] = a[1]
    return Conf(**options)

def gen_flags(parsed:argparse.Namespace) -> str:
    # So all errors are collected together and propagate at once
    err:t.Sequence[Exception] = []

    flags = ["--no-default-features",]
    
    if not parsed.debug:
        flags.append("--release")

    features = []
    if parsed.selinux:
        features.append("selinux")
    if parsed.coreutils:
        features.append("coreutils")
    if parsed.systemd:
        features.append("systemd")
    if len(features) != 0:
        f= ",".join(features)
        flags.append(f"--features={f}")

    if parsed.frozen:
        flags.append("--frozen")
    
    if parsed.native and parsed.target is not None:
        err.append(InvalidArgumentsError("Cannot specify to compile natively and custom target"))
    else: # skip this if it has invalid state for both values
        if parsed.native:
            # cSpell: disable-next-line
            flags.append("-Ctarget-cpu=native")
        else:
            flags.append("--target")
            if parsed.target is None:
                parsed.target = get_host_triple()
            flags.append(parsed.target)


    if err: raise err # raise them here before return
    return " ".join(flags)
def rtarget() -> str:
    global BUILDABLES
    b = []
    for build in BUILDABLES:
        b.append(f"$(BUILD_DIR)/{build}")
    return " ".join(b)
def gen_buildables(parsed:argparse.Namespace) -> str:
    global BUILDABLES
    i = []
    for b in BUILDABLES:
        i.append(f"""
$(BUILD_DIR)/{b}:
@echo "Building {b}"
\t$(CARGO) build -p {b} $(FLAGS)
        """)
    return "\n".join(i)
def gen_install(parsed:argparse.Namespace) -> str:
    global BUILDABLES
    i = []
    for b in BUILDABLES:
        """
install-{b}: $(BUILD_DIR)/{b}
\tinstall -Dm0755 -t $(prefix)/sbin
        """
    return "\n".join(i)
def generate_makefile(conf:Conf, parsed:argparse.Namespace) -> None:
    global BUILDABLES
    install_targets = [f"{x}-install" for x in BUILDABLES]
    phoney = " ".join(BUILDABLES) + " " + " ".join(install_targets)
    profile = "debug" if parsed.debug else "release"
    f= f"""
# A generated Makefile for Easyinit. Don't edit this file directly!
# any issues with this file should be investigated in configure.py
RUSTC ?= {conf.rustc}
CARGO ?= {conf.cargo}
PREFIX ?= {parsed.prefix}
FLAGS := {gen_flags(parsed)}
BUILD_DIR := target/{profile}
TARGET ?= {parsed.target}


all: {rtarget()}
\t $(CARGO) build $(FLAGS)

{gen_buildables(parsed)}

install: all ;


clean:
\t$(CARGO) clean

pre-fetch:
\t$(CARGO) fetch --locked --target $(TARGET)

.PHONY: install {phoney} clean pre-fetch
"""
    l.info(f"Writing Makefile to {parsed.output}")
    l.info(f"Makefile contents:\n{f}")
    with open(parsed.output, "w") as mf:
        mf.write(f)



class InvalidArgumentsError(RuntimeError):
    """
    Errors if the user passes invalid flags to the process

    Like passing custom target then specifying that the program should optimize
    for the current system
    """


if __name__ == "__main__":
    main()