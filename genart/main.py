import argparse
import logging
import pkgutil

from genart import __version__

log = logging.getLogger(__name__)


def main():
    args = argparse.ArgumentParser(
        prog="genart",
        description="Generative art playground",
    )
    args.add_argument("--version", action="version", version=__version__)



    args = args.parse_args()

    print("Hello World!")


def subpackage_main_modules():


if __name__ == "__main__":
    main()
