<h1 align="center">elysium</h1>

### dependencies

nightly rust from [rustup](https://rustup.rs).

system packages and their required features (this is for gentoo, but you can figure it out for other distros).

```
dev-libs/glib
media-libs/openal
media-libs/libsdl2 X alsa opengl
net-dns/c-ares
net-misc/curl ssl
x11-libs/pango
# probably more?
```

### running it

as with any other rust program

```
cargo run --
```

logging level is controlled via the environment variable [`RUST_LOG`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html).

### help it no work

in the csgos directory, you may want to remove:

```
bin/linux64/libpango-1.0.so
bin/linux64/libpangoft2-1.0.so
```

as they cause the following:

```
GameTypes: missing mapgroupsSP entry for game type/mode (custom/custom).
GameTypes: missing mapgroupsSP entry for game type/mode (cooperative/cooperative).
GameTypes: missing mapgroupsSP entry for game type/mode (cooperative/coopmission).
Segmentation fault
```

for some reason, the game doesn't omit why to stdout/stderr, but the following is passed to `libtier0_client.so`'s `LoggingSystem_Log`:

```
Unable to load 'libpangoft1-1.0.so' (error info '/usr/lib64/libpng16.so.16: version `PNG12_0' not found (required by bin/linux64/libpangoft2-1.0.so)'), your game install may be corrupted or you may have a system conflict
```
