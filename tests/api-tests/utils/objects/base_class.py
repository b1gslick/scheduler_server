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
        auth: dict[str, str] | None = None,
    ) -> Response:
        return requests.get(
            f"{self.base_url}/{path}",
            headers=self._get_auth_header(auth),
        )

    def _post(
        self,
        path: str,
        body: Any,
        auth: dict[str, str] | None = None,
    ) -> Response:
        return requests.post(
            f"{self.base_url}/{path}",
            data=body,
            headers=self._get_auth_header(auth),
        )

    def _put(
        self,
        path: str,
        _id: int,
        body: Any,
        auth: dict[str, str] | None = None,
    ) -> Response:
        return requests.put(
            f"{self.base_url}/{path}/{_id}",
            data=body,
            headers=self._get_auth_header(auth),
        )

    def _delete(
        self,
        path: str,
        _id: int,
        auth: dict[str, str] | None = None,
    ) -> Response:
        return requests.delete(
            f"{self.base_url}/{path}/{_id}",
            headers=self._get_auth_header(auth),
        )

    def _get_auth_header(
        self,
        auth: dict[str, str] | None,
    ) -> dict[str, str]:
        return (
            auth
            if auth is not None
            else {
                "Authorization": self.token,
            }
        )
