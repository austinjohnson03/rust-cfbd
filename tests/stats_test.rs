use rust_cfbd::models::cfb::entity::player_stats::PlayerStat;
use rust_cfbd::models::cfb::entity::season_type::SeasonType;
use rust_cfbd::models::cfb::entity::team_stat::TeamStat;
use rust_cfbd::models::cfb::query::stat_season_query::StatSeasonQueryBuilder;
use rust_cfbd::models::cfb::query::stats_player_season_query::StatsPlayerSeasonQueryBuilder;

mod common;

#[tokio::test]
async fn test_stats_season_conference_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = StatSeasonQueryBuilder::new()
        .year(2023)
        .conference("sec")
        .build();
    let result = client.get_season_stats(&query).await.unwrap();

    let expected: Vec<TeamStat> =
        serde_json::from_str(include_str!("common/fixtures/stats_season_conference.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_stats_season_week_range_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = StatSeasonQueryBuilder::new()
        .year(2023)
        .start_week(2)
        .end_week(6)
        .build();
    let result = client.get_season_stats(&query).await.unwrap();

    let expected: Vec<TeamStat> =
        serde_json::from_str(include_str!("common/fixtures/stats_season_week_range.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_stats_season_required_year_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = StatSeasonQueryBuilder::new().year(2023).build();
    let result = client.get_season_stats(&query).await.unwrap();

    let expected: Vec<TeamStat> = serde_json::from_str(include_str!(
        "common/fixtures/stats_season_required_year.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_stats_season_required_team_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = StatSeasonQueryBuilder::new().team("alabama").build();
    let result = client.get_season_stats(&query).await.unwrap();

    let expected: Vec<TeamStat> = serde_json::from_str(include_str!(
        "common/fixtures/stats_season_required_team.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_stats_player_season_season_type_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = StatsPlayerSeasonQueryBuilder::new()
        .year(2023)
        .season_type(SeasonType::Postseason)
        .build();
    let result = client.get_player_season_stats(&query).await.unwrap();

    let expected: Vec<PlayerStat> = serde_json::from_str(include_str!(
        "common/fixtures/stats_player_season_season_type.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_stats_player_season_week_range_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = StatsPlayerSeasonQueryBuilder::new()
        .year(2023)
        .start_week(2)
        .end_week(6)
        .build();
    let result = client.get_player_season_stats(&query).await.unwrap();

    let expected: Vec<PlayerStat> = serde_json::from_str(include_str!(
        "common/fixtures/stats_player_season_week_range.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_stats_player_season_conference_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = StatsPlayerSeasonQueryBuilder::new()
        .year(2023)
        .conference("sec")
        .build();
    let result = client.get_player_season_stats(&query).await.unwrap();

    let expected: Vec<PlayerStat> = serde_json::from_str(include_str!(
        "common/fixtures/stats_player_season_conference.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_stats_player_season_category_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = StatsPlayerSeasonQueryBuilder::new()
        .year(2023)
        .category("passingTDs")
        .build();
    let result = client.get_player_season_stats(&query).await.unwrap();

    let expected: Vec<PlayerStat> = serde_json::from_str(include_str!(
        "common/fixtures/stats_player_season_category.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_stats_player_season_team_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = StatsPlayerSeasonQueryBuilder::new()
        .year(2023)
        .team("alabama")
        .build();
    let result = client.get_player_season_stats(&query).await.unwrap();

    let expected: Vec<PlayerStat> = serde_json::from_str(include_str!(
        "common/fixtures/stats_player_season_team.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_stats_player_season_required_year_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = StatsPlayerSeasonQueryBuilder::new().year(2023).build();
    let result = client.get_player_season_stats(&query).await.unwrap();

    let expected: Vec<PlayerStat> = serde_json::from_str(include_str!(
        "common/fixtures/stats_player_season_required_year.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}
