import random
from requests import Response

from utils.objects.base import BaseClass
from utils.objects.types import ActivityType
from utils.utils import generate_random_string


class Activity(BaseClass):
    def __init__(self, base_url: str, token: str):
        super().__init__(base_url, token)

    def create(
        self,
        title: str | None = None,
        content: str | None = None,
        time: int | None = None,
        auth_header: dict[str, str] | None = None,
    ) -> Response:
        activity = ActivityType(
            title=title if title else generate_random_string(64),
            content=content if content else generate_random_string(400),
            time=time if time else random.randint(0, 100),
        )
        return self.post("activity", activity.model_dump_json(), auth=auth_header)

    def get_one(
        self,
        _id: int,
        auth_header: dict[str, str] | None = None,
    ) -> Response:
        return self.get(f"activity/{_id}", auth=auth_header)

    def get_all(
        self,
        params: dict[str, str] | None = None,
        auth_header: dict[str, str] | None = None,
    ) -> Response:
        return self.get("activity", params=params, auth=auth_header)

    def update(
        self,
        _id: int,
        title: str | None = None,
        content: str | None = None,
        time: int | None = None,
        auth_header: dict[str, str] | None = None,
    ) -> Response:
        activity = ActivityType(
            title=title if title else generate_random_string(64),
            content=content if content else generate_random_string(400),
            time=time if time else random.randint(0, 100),
        )
        return self.put("activity", _id, activity.model_dump_json(), auth=auth_header)

    def delete(
        self,
        _id: int,
        auth_header: dict[str, str] | None = None,
    ) -> Response:
        return self._delete("activity", _id, auth=auth_header)
