from utils.objects.client import Client


def test_user_can_add_activity_update_id_then_delete_id_should_return_empty_list_for_all_activity(
    test_client: Client,
) -> None:
    add = test_client.activity.create()
    assert add.status_code == 201

    activity = test_client.activity.get_last_added()

    assert activity is not None, "can't get last activity after created"
    assert activity.id is not None, (
        f"last activity doesn't have id actual is: {activity}"
    )

    activity.title = "new title"

    update = test_client.activity.update(activity.id, activity)
    assert update.status_code == 201

    delete = test_client.activity.delete(activity.id)
    assert delete.status_code == 200

    assert test_client.activity.get_all().json() == []
