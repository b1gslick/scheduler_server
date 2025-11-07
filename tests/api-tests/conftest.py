import os
from typing import Any, Generator
import pytest
from testcontainers.core.network import Network

from scheduler_server import SchedulerService
from utils.objects.client import Client

from testcontainers.postgres import PostgresContainer
from testcontainers.redis import RedisContainer


@pytest.fixture(scope="function")
def test_client(base_url):
    return Client(base_url)


@pytest.fixture
def base_url():
    return os.environ.get("BASE_URL", "http://localhost:8080/v1")


network = Network().create()

is_containers = os.environ.get("IS_CONTAINERS") == "true"


@pytest.fixture(scope="session", autouse=is_containers)
def database_container_setup() -> Generator[PostgresContainer, Any, Any]:
    port = 5432
    with (
        PostgresContainer(
            "postgres:16.2-alpine3.18",
            port=port,
            username="postgres",
            password="postgres",
            dbname="postgres",
            driver="postgresql",
        )
        .with_bind_ports(port, port)
        .with_network(network)
        .with_network_aliases("postgres") as pg
    ):
        os.environ["DATABASE_PORT"] = str(pg.get_exposed_port(port))
        yield pg


@pytest.fixture(scope="session", autouse=is_containers)
def cache_container_setup() -> Generator[RedisContainer, Any, Any]:
    port = 6379
    with (
        RedisContainer(
            "redis:8.2.1-alpine",
            port=port,
        )
        .with_bind_ports(port, port)
        .with_network(network)
        .with_network_aliases("redis") as redis
    ):
        yield redis


@pytest.fixture(scope="session", autouse=is_containers)
def server_setup() -> Generator[SchedulerService, Any, Any]:
    port = 8000
    tag = os.environ.get("SERVER_TAG", "main")
    os.environ["BASE_URL"] = f"http://localhost:{port}/v1"
    with (
        SchedulerService(
            f"t1mon1106/scheduler:{tag}",
            port=port,
            db_pass="postgres",
            db_name="postgres",
        )
        .with_bind_ports(port, port)
        .with_network(network)
        .with_network_aliases("server") as server
    ):
        yield server
