.DEFAULT_GOAL := run_test
.PHONY: clean

ifneq (,$(wildcard ./.env))
    include .env
    export
endif

cmd-exists-%:
	@hash $(*) > /dev/null 2>&1 || \
		(echo "ERROR: '$(*)' must be installed and available on your PATH."; exit 1)

#initdb -U postgres -D /home/timon/Public/scheduler_server/integration-tests/test_cluster --encoding=UTF-8 --locale=en_US.utf8
#postgres=# create database test;
#postgres=# create user username with encrypted password 'password';
#postgres=# grant all privileges on database test to username;
#GRANT CREATE ON SCHEMA public TO username;

psql: cmd-exists-psql
	psql "postgres://${DATABASE_USER}:${DATABASE_PASSWORD}@${DATABASE_HOST}:${DATABASE_PORT}/${DATABASE_NAME}" &
	psql -U postgres -d ${DATABASE_NAME} -c "TRUNCATE TABLE accounts;" || true
	psql -U postgres -d ${DATABASE_NAME} -c "TRUNCATE TABLE time_spent;" || true
	psql -U postgres -d ${DATABASE_NAME} -c "TRUNCATE TABLE activities CASCADE;" || true


up_db:
	pg_ctl -D test_cluster -l test_cluster/log start

run_test: clean up_db create_db psql
	cargo run || true

create_db:
	psql -U postgres -c "CREATE DATABASE test OWNER username" || true

clean:
	pg_ctl stop -D test_cluster || true
	echo "Finish"
