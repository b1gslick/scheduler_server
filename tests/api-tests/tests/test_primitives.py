from utils.objects.client import Client
from utils.objects.types import ActivityType


def test_get_all_activities_with_empty_state_should_return_empty_list(
    test_client: Client,
) -> None:
    response = test_client.activity.get_all()

    assert response.status_code == 200
    assert response.json() == []


def test_create_activity_should_return_body_with_activity_and_201(
    test_client: Client,
) -> None:
    response = test_client.activity.create(
        title="test_title", content="test_content", time=100
    )
    activity: ActivityType = ActivityType.model_validate(response.json())
    assert response.status_code == 201
    assert activity.title == "test_title"
    assert activity.content == "test_content"
    assert activity.time == 100


def test_user_could_update_exist_activity(
    test_client: Client,
) -> None:
    create_activity = test_client.activity.create()
    activity = test_client.activity.get_last_added()

    assert activity is not None, f"can't added activity {create_activity.text}"
    assert activity.id is not None

    update = test_client.activity.update(activity.id, title="updated")
    assert update.status_code == 201

    updated: ActivityType = ActivityType.model_validate(update.json())
    assert updated.title == "updated", (
        f"activity not updated old value is {updated.title}"
    )
