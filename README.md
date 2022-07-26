# Crust

A Minecraft server implementation written in Rust and Elixir

## Supported versions

For now Crust can only accept 1.8 versions, but my goal is to accept every stable release from 1.8 to 1.19.

## Compiling and running

Requirements:

- [Rust](https://www.rust-lang.org/)
- [Elixir](https://elixir-lang.org/)

Run the following commands:

From the root:

```sh
cargo run --release
# If you want the debug logs, you may also run:
# RUST_LOG=debug cargo run --release
```

From the `proxy/` directory:

```
mix run --no-halt
```

## Structure

Crust is made out of two components:

- A reverse proxy written in Elixir that stores connections to Minecraft clients
- A server written in Rust that reads and writes packets from and to Minecraft clients through the proxy

### Why Rust

[Rust](https://www.rust-lang.org/) is both fast and safe because of its [ownership model](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) that avoids using a garbage collector and manually allocating memory which can cause bugs, so I think it's a great choice for this project

### Why Elixir

Although Rust is extremely fast I don't think it's ideal to handle many concurrent connections, as it relies on OS threads. Elixir on the other hand uses processes hosted on the BEAM virtual machine, which are lightweight, fast and designed for massive concurrency.

