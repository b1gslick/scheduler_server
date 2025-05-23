ARG RUST_VERSION=1.83
ARG APP_NAME=server
ARG TARGET=x86_64-unknown-linux-musl
ARG PORT=8080
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
ARG TARGET

ARG PORT

WORKDIR /app

RUN apt-get update && \
  apt-get install -y pkg-config make g++ libssl-dev musl-tools musl-dev build-essential gcc-x86-64-linux-gnu curl && \
  rustup target add ${TARGET}

# For a musl build on M1 Macs, these ENV variables have to be set
ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
ENV CC='gcc'
ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
ENV CC_x86_64-unknown-linux-musl=x86_64-linux-gnu-gcc

RUN --mount=type=bind,source=src,target=src \
  --mount=type=bind,source=handle-errors,target=handle-errors \
  --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
  --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  --mount=type=bind,source=migrations,target=migrations \
  <<EOF
set -e
cargo build --release --target ${TARGET}
cp ./target/${TARGET}/release/${APP_NAME} /bin/server
EOF

FROM debian:bullseye-slim AS final

ARG PORT

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

COPY --from=build /bin/server /bin/

EXPOSE ${PORT}

CMD ["/bin/server"]
