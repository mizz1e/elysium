<h1 align="center"><code>elysium</code></h1>

![screenshot of local game on de_mirage_cyberpunk](assets/unknown.png)

### installation

```bash
# in the directory of where you git clone'd this to

# build the client
$ cargo build --release

# copy the client into csgo's dir (dependencies resolve their paths from the location of the binary)
$ cp "${CARGO_TARGET_DIR:-target}/release/elysium" "${XDG_DATA_HOME:-${HOME}/.local/share}/Steam/steamapps/common/Counter-Strike Global Offensive/elysium"

# run the client
$ ./elysium -steam
```
