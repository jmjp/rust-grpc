FROM rust:slim-buster as build

# create a new empty shell project
RUN USER=root cargo new --bin grpc
WORKDIR /grpc

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./build.rs  ./build.rs
COPY ./src ./src
COPY ./proto ./proto

# install protobuf
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      protobuf-compiler

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/echo_server*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /grpc/target/release/echo-server .

EXPOSE 54321

# set the startup command to run your binary
CMD ["./echo-server", "--server", "[::]", "--port", "54321"]
