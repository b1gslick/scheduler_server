from requests import Response
from utils.objects.base_class import BaseClass
from utils.objects.types import Account


class AuthClient(BaseClass):
    def __init__(self, base_url: str, token: str) -> None:
        super().__init__(base_url, token)

    def registration(self, account: Account) -> Response:
        return self._post(
            "registration",
            body=account.model_dump_json(),
        )

    def login(self, account: Account) -> Response:
        return self._post(
            "login",
            body=account.model_dump_json(),
        )
