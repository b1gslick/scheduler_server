from testcontainers.core.wait_strategies import HttpWaitStrategy
from testcontainers.core.generic import DockerContainer


class SchedulerService(DockerContainer):
    def __init__(
        self,
        image: str = "",
        port: int = 8000,
        db_pass: str = "postgres",
        db_port: int = 5432,
        db_host: str = "postgres",
        db_name: str = "postgres",
        db_user: str = "postgres",
        cache_host: str = "redis",
        cache_port: int = 6379,
        **kwargs,
    ) -> None:
        super().__init__(image=image, **kwargs)
        self.port = port
        self.db_pass = db_pass
        self.db_host = db_host
        self.db_user = db_user
        self.db_port = db_port
        self.db_name = db_name
        self.cache_host = cache_host
        self.cache_port = cache_port

        self.with_exposed_ports(self.port)

        self.waiting_for(
            strategy=HttpWaitStrategy(self.port, "v1/healthz").for_status_code(200),
        )

    def _configure(self) -> None:
        self.with_env("PORT", str(self.port))
        self.with_env("RUST_LOG", "debug")
        self.with_env("PASETO_KEY", "RANDOM WORDS WINTER MACINTOSH PC")
        self.with_env("DATABASE_PASSWORD", self.db_pass)
        self.with_env("DATABASE_HOST", self.db_host)
        self.with_env("DATABASE_USER", self.db_user)
        self.with_env("DATABASE_PORT", str(self.db_port))
        self.with_env("DATABASE_DB", self.db_name)
        self.with_env("CACHE_HOST", self.cache_host)
        self.with_env("CACHE_PORT", str(self.cache_port))
