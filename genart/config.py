import logging
import logging.config
from pathlib import Path

import yaml

log = logging.getLogger(__name__)

CONFIG_FILE = Path(__file__).parent.parent / "config.yml"


def load_config() -> dict:
    try:
        with CONFIG_FILE.open() as f:
            config = yaml.safe_load(f)

            if "logging" in config:
                logging.config.dictConfig(config["logging"])

            return config

    except yaml.YAMLError as e:
        raise ValueError(f"Invalid configuration: {e}")
    except FileNotFoundError:
        log.warning("No config file found. Skipping config.")
        return {}
