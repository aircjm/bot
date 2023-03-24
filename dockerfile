FROM rust:slim-buster as build

WORKDIR /code

COPY . /code

RUN cargo build --release

# Copy the binary into a new container for a smaller docker image
FROM debian:buster-slim

COPY --from=build /code/target/release/bot /
USER root

ENTRYPOINT ["./bot", "--host", "0.0.0.0"]
