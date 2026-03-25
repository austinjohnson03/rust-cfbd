use rust_cfbd::models::cfb::{
    entity::{
        division_classification::DivisionClassification, game::Game, season_type::SeasonType,
    },
    query::game_query::GameQueryBuilder,
};

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
