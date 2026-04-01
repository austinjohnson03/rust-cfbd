use rust_cfbd::models::cfb::entity::venue::Venue;

mod common;

#[tokio::test]
async fn test_team_conference_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let result = client.get_venues().await.unwrap();

    let expected: Vec<Venue> =
        serde_json::from_str(include_str!("common/fixtures/venues_no_params.json")).unwrap();

    assert_eq!(result, expected);
}
