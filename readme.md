<h1 align="center">elysium</h1>

<p align="center"><img src="assets/unknown.png" width="500" /></p>

### installation

this acts like `csgo-linux64` itself, wherein it launches the game (and possibly injects the cheat part).

```bash
# in the directory of where you git clone'd this to

# build the client
$ cargo build --release

# copy the client into csgo's dir (dependencies resolve their paths from the location of the binary)
$ cp "${CARGO_TARGET_DIR:-target}/x86_64-unknown-linux-gnu/release/elysium" "${XDG_DATA_HOME:-${HOME}/.local/share}/Steam/steamapps/common/Counter-Strike Global Offensive/elysium"

# run the client
$ LD_LIBRARY_PATH="./bin/linux64" ./elysium -steam
```
