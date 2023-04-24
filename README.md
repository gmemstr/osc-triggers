# osc-triggers

Easily map OSC events to keypresses.

## building

Nix is primarily used to build this applicaiton. [InputBot](https://github.com/obv-mikhail/InputBot) is used for keypresses and requires the following dependencies on Linux:

* **libx11-dev**
* **libxtst-dev**
* **libudev-dev**
* **libinput-dev**

It also requires running as root on Linux. For ✨ reasons ✨.

Without Nix, you can just build with the usual `cargo build`.

## configuration

Configuration is fairly straightforward, using TOML to make it human-friendly until a reasonable
user interface can be figured out. An optional value to match can be provided if you want to
trigger different keys for different values.

```toml
[[mapping]]
event = "/some/example"
key = "F24"

[[mapping]]
event = "/some/other/example"
key = "F23"
value = "0"
```

Events come from whatever OSC client you're using. This application is primarily targetting
VRChat but theoretically any will work.

Optionally the port the server runs on can be set.

```toml
[server]
port = 9090
```

The server will always bind to `127.0.0.1` / `localhost`.

## ci/cd & mirroring to github

Development of this primarily takes place over on [sourcehut](https://git.sr.ht/~gmem/osc-triggers). Pull requests and issues are welcome to GitHub though, and your patches will be commited
to sourcehut with proper attribution. The repository is mirrored to GitHub on push to leverage
GitHub Actions for releases. All other testing goes through sourcehut.
