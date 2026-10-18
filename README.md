<img src="icon.png" width="96" align="right" alt="">

# ChickenWM Core

The shared library of ChickenWM: the `chickenwm.conf` parser and lossless writer, the CSS and theme engine, IPC over named pipes, monitor and refresh-rate discovery, the frame scheduler and the emergency failsafe. Every other ChickenWM program builds on it.

Part of [ChickenWM](https://github.com/despale-code/chickenwm), a Hyprland-style tiling desktop for
Windows. Each program lives in its own repository; the ChickenWM repository ties them together
and has the full documentation and the installer.

This project is vibecoded: written with an AI assistant.

## Building

Windows 10 or 11 (64-bit) and Rust 1.85 or newer:

```
cargo build --release
```

## License

MIT, see [LICENSE](LICENSE).
