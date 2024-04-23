export RUST_LOG=debug

default:
	cargo run --bin activities-scheduler-server -- \
		--database-host postgres_container \
		--log-level warn \
		--database-name schedulerdb \
		--database-port 5432

