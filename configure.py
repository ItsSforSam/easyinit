#!/bin/env python3
from __future__ import annotations
from warnings import warn
import typing as t
import os
from pathlib import Path
import sys
import tomllib
import argparse

ROOT = Path(os.path.dirname(__file__))
__version__ = "0.0.1"

def main():
    parsed, conf =  get_parser().parse_known_args()

def get_version() -> str:
    with open(ROOT / "Cargo.toml", "rb") as f:
        data = tomllib.load(f)
    return data["workspace"]["package"]["version"]


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
        default=ROOT / "Makefile.config",
        type=Path,
        metavar="PATH"
    )
    parser.add_argument(
    "--systemd",
    help="Allow systemd integration. Default is when systemd is detected on the system. ()",
    default=is_systemd(),
    action=OptionalFeature,
    )
    return parser


def is_systemd() -> bool:
    """
    Check if systemd is available on this system
    
    If systemctl is in PATH, we assume systemd is available.
    This is to prevent leaving a system stranded without proper service files.

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
                setattr(namespace, self.dest, True)
            elif option_string.startswith("--disable-") or option_string.startswith("--without-"):
                setattr(namespace, self.dest, False)
            # Neither, use default
    
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


def generate_makefile(conf:Conf, parsed:argparse.Namespace) -> None:
    f= f"""
    # A generated Makefile for Easyinit. Don't edit this file directly!
    # any issues with this file should be investigated in configure.py
    RUSTC = {conf.rustc}
    CARGO = {conf.cargo}
    PREFIX = {parsed.prefix}
    # FLAGS := 
    """

if __name__ == "__main__":
    main()