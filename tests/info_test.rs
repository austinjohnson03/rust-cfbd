use rust_cfbd::models::shared::entity::info::UserInfo;

mod common;

#[tokio::test]
async fn test_info_no_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let result = client.get_user_info().await.unwrap();

    let expected: UserInfo =
        serde_json::from_str(include_str!("common/fixtures/info_no_params.json")).unwrap();

    assert_eq!(result, expected);
}
