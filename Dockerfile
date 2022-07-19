FROM alpine:3.16

RUN apk update
RUN apk add curl build-base pkgconf libzmq-static libsodium-static zeromq-dev \
    openssl-dev cmake protoc git rsync nodejs-current npm rust cargo

# Install `task` as a modern `make`
RUN sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d -b /usr/local/bin

# Force static compilation
ENV RUSTFLAGS="-C target-feature=+crt-static"

COPY entrypoint.sh /entrypoint.sh

VOLUME /data
WORKDIR /data
ENTRYPOINT [ "/entrypoint.sh" ]