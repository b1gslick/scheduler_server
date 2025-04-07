from utils.objects.client import Client


def test_user_create_activity_update_then_delete_it_activity_should_be_deleted(
    test_client: Client,
) -> None:
    add = test_client.activity.create()
    assert add.status_code == 201

    act = test_client.activity.get_all().json()

    update = test_client.activity.update(act[0]["id"], title="new title")
    assert update.status_code == 201
    assert update.json()["title"] == "new title"

    resp = test_client.activity.delete(act[0]["id"])
    assert resp.status_code == 200
    assert test_client.activity.get_all().json() == []
