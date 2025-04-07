from utils.objects.client import Client


def test_get_all_activities_with_empty_state_should_return_empty_list(
    test_client: Client,
) -> None:
    response = test_client.activity.get_all()
    assert response.status_code == 200
    assert response.json() == []


def test_create_activity_should_return_201_and_activity_body(
    test_client: Client,
) -> None:
    response = test_client.activity.create(title="test")
    assert response.status_code == 201
    assert response.json()["title"] == "test"
