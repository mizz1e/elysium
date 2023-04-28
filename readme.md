<h1 align="center">elysium</h1>

elysium is a csgo client launcher for linux

### dependencies

elysium

 - rust nightly, preferably from [rustup](https://rustup.rs)
 - clang, for generating bindings of source structiures
 - sdl v2, to link against, and hook methods

csgo itself (may be optional, may be not)

 - openal, for audio
 - curl, why does engine client use curl of all things?
 - pango, panorama font rendering
 - sdl v2, hence elysium hooks sdl methods
 - tcmalloc, using a fancy allocator doesnt stop uafs
 - probably more

### building

```bash
cargo build --release
```

backup the "stock" `csgo_linux64`, probably? replace the original with `target/release/csgo_linux64`

### dependencies

nightly rust from [rustup](https://rustup.rs).

system packages and their required features (this is for gentoo, but you can figure it out for other distros).

run it as usual? or just do

```bash
LD_LIBRARY_PATH=bin/linux64:csgo/bin/linux64 ./csgo_linux64
```

logging level is controlled via the environment variable [`RUST_LOG`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html).

### help it no work

in the csgo directory, you may want to remove the provided pango

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

for some reason, the game doesn't omit why to stdout/stderr, but the following is passed to `libtier0_client.so`'s `LoggingSystem_Log`

```
Unable to load 'libpangoft1-1.0.so' (error info '/usr/lib64/libpng16.so.16: version `PNG12_0' not found (required by bin/linux64/libpangoft2-1.0.so)'), your game install may be corrupted or you may have a system conflict
```
