<h1 align="center">Elysium</h1>

Counter Strike: Global Offensive Client for Linux.

### Dependencies

 - Nightly [Rust](https://rustup.rs).
 - LLVM's Clang, for [bindgen](https://docs.rs/bindgen).
 - SDL 2, to easily hook methods, and map events.

### Usage

Build it like any other Rust program, a `csgo_linux64` binary will be produced. Replace the binary by the same name in CS:GO's directory. (If you didn't back up the original, verify the game files.) Because Source engine, run the client like this, from the game's directory.

```bash
LD_LIBRARY_PATH=bin/linux64:csgo/bin/linux64 ./csgo_linux64
```

Logging level is controlled via [`RUST_LOG`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html).
