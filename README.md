# Statically compiling Rust binaries

This is a recipe for statically compiling a Rust binary that includes native dependencies (i.e., an external library). This repository includes a sample application using ZeroMQ. The binary generated with this image can be distributed without the need of including `libzmq` or any other dependency.

## Building the image

Simply build from the provided `docker-compose.yaml` file:

```bash
git clone ...
cd alpine-rust
sudo docker-compose build
```

## Compiling the sample app

To compile the sample app, run:

```bash
sudo docker-compose run alpine-rust
```

Obtain the binary at `sample/target/x86_64-alpine-linux-musl/debug/sample`. Check that it is self-contained with:

```bash
ldd sample/target/x86_64-alpine-linux-musl/debug/sample
# should print `statically linked`
```

You can pass the option `--release` to the `docker-compose` invocation, to build a release version of the binary.

## Compiling your Rust app

- Add any necessary native library to the `Dockerfile` and rebuild the container.

- Update (or create) the file `build.rs` at the root of your Rust app (check the provided example `sample/build.rs`). This file needs to declare all the native libraries to be statically compiled. This process can include some trial and error until `build.rs` lists all the libraries (notice that the sample app needs to include `libsodium` and `libstdc++`, even though the only "apparent" dependency is `libzmq`).

- Run
    ```bash
    sudo SRC=/path/to/my/app docker-compose run alpine-rust  # optionally, add `--release`