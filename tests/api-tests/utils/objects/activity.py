import random
from requests import Response
from utils.helpers import generate_random_string
from utils.objects.base_class import BaseClass
from utils.objects.types import ActivityType


class Activity(BaseClass):
    def __init__(self, base_url: str, token: str) -> None:
        super().__init__(base_url, token)

    def create(
        self,
        title: str = generate_random_string(64),
        time: int = random.randint(0, 1000),
        content: str = generate_random_string(500),
        auth_header: dict[str, str] | None = None,
    ) -> Response:
        activity = ActivityType(
            title=title,
            content=content,
            time=time,
        )
        return self._post(
            "activity",
            body=activity.model_dump_json(),
            auth=auth_header,
        )

    def get_one(
        self,
        _id: int,
        auth_header: dict[str, str] | None = None,
    ) -> Response:
        return self._get(
            f"activity/{_id}",
            auth=auth_header,
        )

    def get_all(
        self,
        params: dict[str, str] | None = None,
        auth_header: dict[str, str] | None = None,
    ) -> Response:
        return self._get(
            "activity",
            params=params,
            auth=auth_header,
        )

    def get_last_added(self) -> ActivityType | None:
        activities = self.get_all().json()
        if len(activities) < 1:
            return None
        return ActivityType.model_validate(activities[-1])

    def update(
        self,
        _id: int,
        title: str = generate_random_string(64),
        time: int = random.randint(0, 1000),
        content: str = generate_random_string(500),
        auth_header: dict[str, str] | None = None,
    ) -> Response:
        activity = ActivityType(
            title=title,
            content=content,
            time=time,
        )
        return self._put(
            "activity",
            _id=_id,
            body=activity.model_dump_json(),
            auth=auth_header,
        )

    def delete(
        self,
        _id: int,
        auth_header: dict[str, str] | None = None,
    ) -> Response:
        return self._delete("activity", _id, auth=auth_header)
