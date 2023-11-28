FROM rust:latest

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/activities-scheduler-server"]

EXPOSE 8080
