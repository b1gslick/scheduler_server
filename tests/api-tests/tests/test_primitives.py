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
    assert activity.time == 100 * 60


def test_user_could_update_exist_activity(
    test_client: Client,
) -> None:
    create_activity = test_client.activity.create()
    activity = test_client.activity.get_last_added()

    assert activity is not None, f"can't added activity {create_activity.text}"
    assert activity.id is not None

    activity.title = "updated"

    update = test_client.activity.update(activity.id, activity)
    assert update.status_code == 201

    updated: ActivityType = ActivityType.model_validate(update.json())
    assert updated.title == "updated", (
        f"activity not updated old value is {updated.title}"
    )

    new_activity = test_client.activity.get_one(activity.id)

    assert new_activity.status_code == 200
    new: ActivityType = ActivityType.model_validate(new_activity.json())

    assert new.title == "updated"
    assert new.content == activity.content
    assert new.time == activity.time * 60
