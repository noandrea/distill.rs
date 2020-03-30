FROM rust:1-slim as build

# maybe we can optimize this but easiest way to get nightly
RUN rustup default nightly
RUN USER=root cargo new --bin --vcs none aeternal
WORKDIR /distill

RUN apt-get update && apt-get -y install \
      pkg-config \
      libssl-dev \
      libcurl3-dev \
      libcurl4-openssl-dev

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN touch ./src/lib.rs

RUN cargo build --release
RUN rm src/*.rs && rm -rf target/release/deps/distill*
COPY ./src/ ./src/
COPY ./migrations/ ./migrations/
COPY ./swagger/ ./swagger/
RUN cargo build --release

FROM debian:buster-slim
ENV DEBIAN_FRONTEND noninteractive
# RUN apt-get update && apt-get -y install libpq5 libcurl4
COPY --from=build /distill/target/release/distill /app/distill
WORKDIR /app
# copy rocket configuration 
COPY ./docker/Rocket.toml ./Rocket.toml
# copy minimal logger configuration
COPY ./docker/log4rs-minimal.yaml /logs/log.yaml
# copy the wait-for-it.sh script
COPY ./docker/wait-for-it.sh /bin/wait-for-it.sh
# set the entry-point
ENTRYPOINT ["/app/distill"]
CMD ["-p", "-s", "-w"]
EXPOSE 2003

