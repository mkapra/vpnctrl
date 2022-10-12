FROM rust:latest as build

ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y libc-dev libpq-dev

WORKDIR /code
COPY ./ ./
RUN cargo build --release

FROM ubuntu:latest

RUN apt update && apt install -y libc-dev libpq-dev wireguard
COPY --from=build /code/target/release/api /wgapi
COPY --from=build /code/api/Rocket.toml /Rocket.toml

CMD ["/wgapi"]