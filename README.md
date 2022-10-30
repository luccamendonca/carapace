# Carapace

Run shell commands remotely using gRPC

## Dependencies

- [Rust](https://github.com/rust-lang/rust)
- [Cargo](https://github.com/rust-lang/cargo)

On Debian-based Linux systems:
```sh
$ sudo apt install -y protobuf-compiler libprotobuf-dev
```

## Running

> The `ALLOWED_CMD_PATH` specifies where the application will be allowed to run
> executable binaries/scripts/etc.

### Server
```sh
ALLOWED_CMD_PATH=~/.local/bin make server
```

### Client
```sh
make client
```
