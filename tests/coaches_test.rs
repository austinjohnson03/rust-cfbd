mod common;

use rust_cfbd::models::cfb::entity::coach::Coach;
use rust_cfbd::models::cfb::query::coach_query::CoachQueryBuilder;

#[tokio::test]
async fn test_coaches_first_name_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = CoachQueryBuilder::new().first_name("nick").build();
    let result = client.get_coaches(&query).await.unwrap();

    let expected: Vec<Coach> =
        serde_json::from_str(include_str!("common/fixtures/coaches_first_name.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_coaches_last_name_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = CoachQueryBuilder::new().last_name("bryant").build();
    let result = client.get_coaches(&query).await.unwrap();

    let expected: Vec<Coach> =
        serde_json::from_str(include_str!("common/fixtures/coaches_last_name.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_coaches_max_year_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = CoachQueryBuilder::new().max_year(1980).build();
    let result = client.get_coaches(&query).await.unwrap();

    let expected: Vec<Coach> =
        serde_json::from_str(include_str!("common/fixtures/coaches_max_year.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_coaches_min_year_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = CoachQueryBuilder::new().min_year(2018).build();
    let result = client.get_coaches(&query).await.unwrap();

    let expected: Vec<Coach> =
        serde_json::from_str(include_str!("common/fixtures/coaches_min_year.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_coaches_team_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = CoachQueryBuilder::new().team("alabama").build();
    let result = client.get_coaches(&query).await.unwrap();

    let expected: Vec<Coach> =
        serde_json::from_str(include_str!("common/fixtures/coaches_team.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_coaches_year_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = CoachQueryBuilder::new().year(2024).build();
    let result = client.get_coaches(&query).await.unwrap();

    let expected: Vec<Coach> =
        serde_json::from_str(include_str!("common/fixtures/coaches_year.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_coaches_no_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = CoachQueryBuilder::new().build();
    let result = client.get_coaches(&query).await.unwrap();

    let expected: Vec<Coach> =
        serde_json::from_str(include_str!("common/fixtures/coaches_no_params.json")).unwrap();

    assert_eq!(result, expected);
}
