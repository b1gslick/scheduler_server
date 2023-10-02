FROM rust:1.70.0

COPY . /app

WORKDIR /app

RUN cargo build 

CMD ["./target/debug/scheduler-service"]
