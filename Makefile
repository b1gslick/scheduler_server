export RUST_LOG=debug

default:
	cargo run --bin server -- \
		--database-host 0.0.0.0 \
		--log-level warn \
		--database-name schedulerdb \
		--database-port 5432 \
		--database-user scheduler \
		--database-password scheduler

