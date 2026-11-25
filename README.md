# Chaperon

Small cross-platform utility program inspired by [LNKLauncher](https://github.com/Hexaae/LNKLauncher), launching a program and waiting until a given process exits.

## Usage

```sh
chaperon <program path> <process name> [arguments...]
```

Example:

```sh
chaperon ./my-app.exe my-app.exe --flag value
```

The tool starts the program, waits for a process named `<process name>` to appear, then waits until it exits.

## Build

```sh
cargo build --release
```

## License

Copyright (c) Alexandre Breteau <contact@seldszar.fr> (https://seldszar.fr)

This software is released under the terms of the MIT License.
See the [LICENSE](LICENSE.md) file for further information.
