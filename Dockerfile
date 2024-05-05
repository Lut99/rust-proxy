# DOCKERFILE.IMAGE for the `rust-proxy` binary
#   by Lut99


##### BUILD #####
# Start with the Rust image
FROM rust:1-alpine3.19 AS build

# Add additional dependencies
RUN apk add --no-cache \
    musl-dev

# Copy over the source
RUN mkdir -p /source/target
COPY Cargo.toml /source/Cargo.toml
COPY Cargo.lock /source/Cargo.lock
COPY src /source/src

# Build it
WORKDIR /source
RUN --mount=type=cache,id=cargoidx,target=/usr/local/cargo/registry \
    --mount=type=cache,id=rust-proxy,target=/source/target \
    cargo build --release --bin rust-proxy \
 && cp /source/target/release/rust-proxy /source/rust-proxy



##### RELEASE #####
# The release is alpine-based for quickness
FROM alpine:3.19 AS run

# Copy the binary from the build
COPY --from=build /source/rust-proxy /rust-proxy

# Alrighty define the entrypoint and be done with it
ENTRYPOINT [ "/rust-proxy" ]
