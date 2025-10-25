from utils.helpers import generate_random_string
from utils.objects.activity import Activity
from utils.objects.auth import AuthClient
from utils.objects.types import Account, LoginResponse


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
            email=email if email is not None else f"{generate_random_string()}@test.iv",
            password=password if password is not None else "testTT22$$$",
        )

        reg = self.auth.registration(account)
        assert reg.status_code == 201, f"can't registration with {reg.text}"

        login = self.auth.login(account)
        assert login.status_code == 200, f"can't login with {login.text}"

        login_response: LoginResponse = LoginResponse.model_validate(login.json())

        return login_response.token
