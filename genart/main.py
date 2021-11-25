import argparse
import importlib
import logging
import pkgutil
from pathlib import Path
from typing import Iterator

from genart import __version__

log = logging.getLogger(__name__)


def main():
    parser = argparse.ArgumentParser(
        prog="genart",
        description="Generative art playground",
    )
    parser.add_argument("--version", action="version", version=__version__)

    cfg = {}

    subparsers = parser.add_subparsers(dest="subcommand", required=True)

    for sub_pkg in subpackages():
        mod = importlib.import_module(sub_pkg.name)
        if hasattr(mod, "register_parser"):
            mod.register_parser(subparsers)

    args = parser.parse_args()
    args.func(args, cfg)


def subpackages() -> Iterator[pkgutil.ModuleInfo]:
    this_dir = Path(__file__).parent

    for mod in pkgutil.walk_packages([str(this_dir)], prefix="genart."):
        if mod.ispkg:
            yield mod


if __name__ == "__main__":
    main()
