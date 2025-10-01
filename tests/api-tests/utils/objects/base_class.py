from typing import Any
from requests import Response
import requests


class BaseClass:
    def __init__(self, base_url: str, token: str) -> None:
        self.base_url = base_url
        self.token = token

    def _get(
        self,
        path: str,
        params: dict[str, str] | None = None,
        auth: dict[str, str] | None = None,
    ) -> Response:
        auth_header = auth if auth is not None else {"Authorization": self.token}
        return requests.get(
            f"{self.base_url}/{path}",
            headers=auth_header,
            params=params,
        )

    def _post(
        self,
        path: str,
        body: Any,
        auth: dict[str, str] | None = None,
    ) -> Response:
        auth_header = auth if auth is not None else {"Authorization": self.token}
        return requests.post(
            f"{self.base_url}/{path}",
            headers=auth_header,
            data=body,
        )

    def _put(
        self,
        path: str,
        _id: int,
        body: Any,
        auth: dict[str, str] | None = None,
    ) -> Response:
        auth_header = auth if auth is not None else {"Authorization": self.token}
        return requests.put(
            f"{self.base_url}/{path}/{_id}",
            data=body,
            headers=auth_header,
        )

    def _delete(
        self,
        path: str,
        _id: int,
        auth: dict[str, str] | None = None,
    ) -> Response:
        auth_header = auth if auth is not None else {"Authorization": self.token}
        return requests.delete(
            f"{self.base_url}/{path}/{_id}",
            headers=auth_header,
        )
