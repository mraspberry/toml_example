#!/usr/bin/env python3

import json
import pathlib
import toml


def main():
    filepath = pathlib.Path("test.toml")
    config = toml.loads(filepath.read_text())
    print(json.dumps(config, indent=2))


if __name__ == "__main__":
    main()
