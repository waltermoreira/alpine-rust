# Statically compiling Rust binaries

This is a recipe for statically compiling a Rust binary that includes native dependencies (i.e., an external library). This repository includes a sample application using ZeroMQ. The binary generated with this image can be distributed without the need of including `libzmq` or any other dependency.

## Building the image

Simply build from the provided `Dockerfile`:

```bash
git clone ...
cd alpine-rust
sudo docker build -t alpine-rust .
```

## Compiling the sample app

Run

```bash
sudo docker run -v /tmp:/root/.cargo -v $(pwd)/sample:/data -it alpine-rust
```

Obtain the binary at `sample/target/x86_64-alpine-linux-musl/debug/sample`. Check that it is self-contained with:

```bash
ldd sample/target/x86_64-alpine-linux-musl/debug/sample
# should print `statically linked`
```

You can pass the option `--release` to the docker invocation, to build a release version of the binary.