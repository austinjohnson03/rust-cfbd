use rust_cfbd::models::cfb::entity::poll_week::PollWeek;
use rust_cfbd::models::cfb::entity::season_type::SeasonType;
use rust_cfbd::models::cfb::query::ranking_query::RankingQueryBuilder;

mod common;

#[tokio::test]
async fn test_rankings_week_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = RankingQueryBuilder::new().year(2024).week(12).build();
    let result = client.get_rankings(&query).await.unwrap();

    let expected: Vec<PollWeek> =
        serde_json::from_str(include_str!("common/fixtures/rankings_week.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_rankings_season_type_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = RankingQueryBuilder::new()
        .year(2024)
        .season_type(SeasonType::Regular)
        .build();
    let result = client.get_rankings(&query).await.unwrap();

    let expected: Vec<PollWeek> =
        serde_json::from_str(include_str!("common/fixtures/rankings_season_type.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_rankings_required_year_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = RankingQueryBuilder::new().year(2024).build();
    let result = client.get_rankings(&query).await.unwrap();

    let expected: Vec<PollWeek> =
        serde_json::from_str(include_str!("common/fixtures/rankings_required_year.json")).unwrap();

    assert_eq!(result, expected);
}
