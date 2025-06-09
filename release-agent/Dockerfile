ARG RUST_VERSION=1.86
FROM rust:${RUST_VERSION}-bullseye AS dependency
WORKDIR /opt/release-agent

RUN mkdir -p src && echo "fn main() {}" >> src/main.rs

COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo fetch

FROM dependency AS build

RUN apt-get update && apt install --no-install-recommends -y musl-tools clang protobuf-compiler llvm git nettle-dev pkg-config libssl-dev

COPY src src
COPY proto proto
COPY build.rs .
RUN --mount=type=cache,target=/opt/target/ \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml  \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock  \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    NETTLE_STATIC=yes cargo build --release && \
    cp ./target/release/sealci-release-agent /bin/sealci-release-agent

FROM ubuntu:latest AS final

WORKDIR /opt/sealci-release-agent

# Copy the executable from the "build" stage.
COPY --from=build /bin/sealci-release-agent /opt/sealci-release-agent/sealci-release-agent

# What the container should run when it is started.
CMD ["/opt/sealci-release-agent/sealci-release-agent"]
