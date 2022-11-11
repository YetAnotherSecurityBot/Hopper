FROM rust:1.60 as build

# create a new empty shell project
RUN USER=root cargo new --bin hopper
WORKDIR /hopper

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/hopper*
RUN cargo build --release

# our final base
FROM rust:1.60-slim-buster

# copy the build artifact from the build stage
COPY --from=build /hopper/target/release/hopper .

# set the startup command to run your binary
CMD ["./hopper"]