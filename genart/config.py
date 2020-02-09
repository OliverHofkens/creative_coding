import logging
import logging.config
from pathlib import Path

import yaml

log = logging.getLogger(__name__)

PROJECT_ROOT = Path(__file__).parent.parent
CONFIG_FILE = PROJECT_ROOT / "config.yml"


def load_config() -> dict:
    try:
        with CONFIG_FILE.open() as f:
            config = yaml.safe_load(f)

            if "logging" in config:
                logging.config.dictConfig(config["logging"])
                log.debug("Configured logging")

            if "output_dir" in config:
                output_dir = Path(config["output_dir"])
                if not output_dir.is_absolute():
                    output_dir = PROJECT_ROOT / output_dir

                output_dir.mkdir(parents=True, exist_ok=True)
                log.debug("Set output dir to %s", output_dir)
                config["output_dir"] = output_dir

            return config

    except yaml.YAMLError as e:
        raise ValueError(f"Invalid configuration: {e}")
    except FileNotFoundError:
        log.warning("No config file found. Skipping config.")
        return {}
