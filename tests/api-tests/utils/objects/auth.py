from requests import Response

from utils.objects.base import BaseClass
from utils.objects.types import Account


class AuthClient(BaseClass):
    def __init__(self, base_url: str, token: str):
        super().__init__(base_url, token)

    def registration(self, acc: Account) -> Response:
        return self.post("registration", acc.model_dump_json(), auth={})

    def login(self, acc: Account) -> Response:
        return self.post("login", acc.model_dump_json(), auth={})
