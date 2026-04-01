use rust_cfbd::models::cfb::entity::conference::Conference;

mod common;

#[tokio::test]
async fn test_conference_no_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let result = client.get_conferences().await.unwrap();

    let expected: Vec<Conference> =
        serde_json::from_str(include_str!("common/fixtures/conferences_no_params.json")).unwrap();

    assert_eq!(result, expected);
}
