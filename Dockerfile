FROM rust:1.74.1-slim-bullseye as builder
# 1. Create a new empty shell project
RUN USER=root cargo new --bin activities-scheduler-server
WORKDIR /scheduler

COPY ./ ./

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update; apt-get clean

# Install wget.
RUN \
  apt-get install -y wget && \
  apt-get install -y openssl && \
  apt-get install -y gnupg && \
  apt-get install -y gcc
# Set the Chrome repo.
RUN wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add - \
  && echo "deb http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list

# Install Chrome.
RUN apt-get update && apt-get -y install google-chrome-stable

COPY --from=builder ./scheduler/target/release/activities-scheduler-server .
COPY ./setup.toml .

CMD ["./activities-scheduler-server"]
