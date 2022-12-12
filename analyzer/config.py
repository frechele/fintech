import tomllib


DEFAULT_CONFIG_FILENAME = "config.toml"


def load_configuration(filename: str = DEFAULT_CONFIG_FILENAME):
    with open(filename, "rt") as f:
        return tomllib.loads(f.read())
