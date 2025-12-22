# Percepter

Running on a Raspberry Pi, *Percepter* is intended to collect data from sensors and to send these to a server running the *Herodot* service.

## Testing

Tests are run by

```bash
cargo test
```

## Building

Building locally is straightforward with `cargo build`. As the intended end system, the Raspberry Pi, is rather limited, development is expected to be done on a more powerful AMD64 system. Cross building is easily done using the Cross application, which can be installed by executing `cargo install cross`. Doing so enables simple cross compilation. To build for the Raspberry Pi 3B, run

```bash
cross build --target aarch64-unknown-linux-gnu
```

Note that since Cross uses it, Docker must be installed and running.

The build process can be expanded with a single line deployment. Say, a Raspberry Pi is running on the local network with hostname `raspberrypi` and username `pi`, build and deploy with

```bash
cross build --target aarch64-unknown-linux-gnu && scp target/aarch64-unknown-linux-gnu/debug/percepters pi@raspberrypi.local:/home/pi/percepter
```

Using `&&` ensures that the deployment only happens if the build succeeds.
