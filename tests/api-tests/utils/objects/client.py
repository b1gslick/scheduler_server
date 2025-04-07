from utils.objects.activity import Activity
from utils.objects.auth import AuthClient
from utils.objects.types import Account, LoginResponse
from utils.utils import generate_random_string


class Client:
    def __init__(self, base_url: str) -> None:
        self.auth = AuthClient(base_url, "")
        self.token = self.create_user()
        self.activity = Activity(base_url, self.token)

    def create_user(
        self,
        email: str | None = None,
        password: str | None = None,
    ) -> str:
        account = Account(
            email=email
            if email is not None
            else f"{generate_random_string(14)}@test.iv",
            password=password if password is not None else "testTT22$$$",
        )
        resp = self.auth.registration(account)
        assert resp.status_code == 201, f"can't registration with {resp.text}"

        login = self.auth.login(account)
        assert login.status_code == 200, f"can't login with {login.text}"

        login_resp: LoginResponse = LoginResponse.model_validate(login.json())
        return login_resp.token
