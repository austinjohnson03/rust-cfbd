use rust_cfbd::models::cfb::entity::division_classification::DivisionClassification;
use rust_cfbd::models::cfb::entity::game::Game;
use rust_cfbd::models::cfb::entity::game_media::GameMedia;
use rust_cfbd::models::cfb::entity::media_type::MediaType;
use rust_cfbd::models::cfb::entity::season_type::SeasonType;
use rust_cfbd::models::cfb::query::game_media_query::GameMediaQueryBuilder;
use rust_cfbd::models::cfb::query::game_query::GameQueryBuilder;

mod common;

#[tokio::test]
async fn test_games_classification_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = GameQueryBuilder::new()
        .year(2023)
        .classification(DivisionClassification::Fcs)
        .build();
    let result = client.get_games(&query).await.unwrap();

    let expected: Vec<Game> =
        serde_json::from_str(include_str!("common/fixtures/games_classification.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_games_conference_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = GameQueryBuilder::new().year(2023).conference("ACC").build();
    let result = client.get_games(&query).await.unwrap();

    let expected: Vec<Game> =
        serde_json::from_str(include_str!("common/fixtures/games_conference.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_games_home_away_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = GameQueryBuilder::new()
        .year(2023)
        .home("alabama")
        .away("lsu")
        .build();
    let result = client.get_games(&query).await.unwrap();

    let expected: Vec<Game> =
        serde_json::from_str(include_str!("common/fixtures/games_home_away.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_games_season_type_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = GameQueryBuilder::new()
        .year(2023)
        .season_type(SeasonType::Postseason)
        .build();
    let result = client.get_games(&query).await.unwrap();

    let expected: Vec<Game> =
        serde_json::from_str(include_str!("common/fixtures/games_season_type.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_games_team_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = GameQueryBuilder::new().year(2023).team("LSU").build();
    let result = client.get_games(&query).await.unwrap();

    let expected: Vec<Game> =
        serde_json::from_str(include_str!("common/fixtures/games_team.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_games_week_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = GameQueryBuilder::new().year(2023).week(1).build();
    let result = client.get_games(&query).await.unwrap();

    let expected: Vec<Game> =
        serde_json::from_str(include_str!("common/fixtures/games_week.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_games_required_year_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = GameQueryBuilder::new().year(2022).build();
    let result = client.get_games(&query).await.unwrap();

    let expected: Vec<Game> =
        serde_json::from_str(include_str!("common/fixtures/games_required_year.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_games_required_id_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = GameQueryBuilder::new().id(401525434).build();
    let result = client.get_games(&query).await.unwrap();

    let expected: Vec<Game> =
        serde_json::from_str(include_str!("common/fixtures/games_required_id.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_game_media_classification_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = GameMediaQueryBuilder::new()
        .year(2023)
        .classification(DivisionClassification::Fcs)
        .build();
    let result = client.get_game_media(&query).await.unwrap();

    let expected: Vec<GameMedia> = serde_json::from_str(include_str!(
        "common/fixtures/games_media_classification.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_game_media_season_type_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = GameMediaQueryBuilder::new()
        .year(2023)
        .season_type(SeasonType::Postseason)
        .build();
    let result = client.get_game_media(&query).await.unwrap();

    let expected: Vec<GameMedia> =
        serde_json::from_str(include_str!("common/fixtures/games_media_season_type.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_game_media_conference_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = GameMediaQueryBuilder::new()
        .year(2023)
        .conference("sec")
        .build();
    let result = client.get_game_media(&query).await.unwrap();

    let expected: Vec<GameMedia> =
        serde_json::from_str(include_str!("common/fixtures/games_media_conference.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_game_media_media_type_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = GameMediaQueryBuilder::new()
        .year(2023)
        .media_type(MediaType::Tv)
        .build();
    let result = client.get_game_media(&query).await.unwrap();

    let expected: Vec<GameMedia> =
        serde_json::from_str(include_str!("common/fixtures/games_media_media_type.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_game_media_team_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = GameMediaQueryBuilder::new()
        .year(2023)
        .team("alabama")
        .build();
    let result = client.get_game_media(&query).await.unwrap();

    let expected: Vec<GameMedia> =
        serde_json::from_str(include_str!("common/fixtures/games_media_team.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_game_media_week_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = GameMediaQueryBuilder::new().year(2023).week(1).build();
    let result = client.get_game_media(&query).await.unwrap();

    let expected: Vec<GameMedia> =
        serde_json::from_str(include_str!("common/fixtures/games_media_week.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_game_media_required_year_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = GameMediaQueryBuilder::new().year(2023).build();
    let result = client.get_game_media(&query).await.unwrap();

    let expected: Vec<GameMedia> = serde_json::from_str(include_str!(
        "common/fixtures/games_media_required_year.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}
