ARG RUST_VERSION=1.74.1
ARG APP_NAME=activities-scheduler-server
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

RUN apt-get update && \
  apt-get install -y pkg-config make g++ libssl-dev && \
  rustup target add x86_64-unknown-linux-gnu


RUN --mount=type=bind,source=src,target=src \
  --mount=type=bind,source=handle-errors,target=handle-errors \
  --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
  --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  --mount=type=bind,source=migrations,target=migrations \
  <<EOF
set -e
cargo build --locked --release --target x86_64-unknown-linux-gnu
cp ./target/x86_64-unknown-linux-gnu/release/$APP_NAME /bin/server
EOF
FROM debian:bullseye-slim AS final

# create simple user
ARG UID=10001
RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "${UID}" \
  appuser
USER appuser

# copy binaries
COPY --from=build /bin/server /bin/
# copy configuration file
COPY ./setup.toml ./setup.toml

# expose port
EXPOSE 8080

CMD ["/bin/server"]
