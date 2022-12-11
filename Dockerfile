FROM rust:1.60 as build

# create a new empty shell project
RUN USER=root cargo new --bin hopper && echo "Created project"
WORKDIR /hopper

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release && echo "Built dependencies"
RUN rm src/*.rs && echo "Removed src files"

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/hopper* && echo "Removed old binaries"
RUN cargo build --release && echo "Built binary"

# our final base
FROM rust:1.60-slim-buster

# copy the build artifact from the build stage
COPY --from=build /hopper/target/release/hopper .

# set rocket envs
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=27323
ENV ROCKET_ENV=prod
EXPOSE 27323
VOLUME yasb:/logs
RUN echo "Running hopper"
# set the startup command to run your binary
CMD ["./hopper"]