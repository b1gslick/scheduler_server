from utils.objects.client import Client
from utils.objects.types import ActivityType


def test_get_all_activities_with_empty_state_should_return_empty_list(
    test_client: Client,
) -> None:
    response = test_client.activity.get_all()

    assert response.status_code == 200
    assert response.json() == []


def test_create_activity_should_return_201_and_activity_body(
    test_client: Client,
) -> None:
    content = "my new awesome activity"
    title = "my new awesome title"
    time = 100
    response = test_client.activity.create(
        title=title,
        content=content,
        time=time,
    )
    activity_response: ActivityType = ActivityType.model_validate(response.json())
    assert response.status_code == 201
    assert activity_response.title == title
    assert activity_response.content == content
    assert activity_response.time == time * 60


def test_user_could_update_exist_activity(test_client: Client) -> None:
    created = test_client.activity.create()
    assert created.status_code == 201

    activity = test_client.activity.get_last_added()

    assert activity is not None, "Activity list is empty after added"
    assert activity.id is not None, "Can't find id in activity"

    activity.title = "updated"

    update = test_client.activity.update(activity.id, activity)
    assert update.status_code == 201

    after_update = test_client.activity.get_one(activity.id)

    updated_activity: ActivityType = ActivityType.model_validate(after_update.json())

    assert updated_activity.title == "updated", (
        f"activity not updated value is {updated_activity.title}"
    )

    assert updated_activity.content == activity.content
    assert updated_activity.time == activity.time * 60
