use rust_cfbd::models::cfb::entity::division_classification::DivisionClassification;
use rust_cfbd::models::cfb::entity::play::Play;
use rust_cfbd::models::cfb::entity::season_type::SeasonType;
use rust_cfbd::models::cfb::query::play_query::PlayQueryBuilder;

mod common;

#[tokio::test]
async fn test_plays_off_conference_def_conference_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = PlayQueryBuilder::new()
        .year(2024)
        .week(1)
        .offense_conference("acc")
        .defense_conference("sec")
        .build();
    let result = client.get_plays(&query).await.unwrap();

    let expected: Vec<Play> = serde_json::from_str(include_str!(
        "common/fixtures/plays_off_conference_def_conference.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_plays_offense_defense_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = PlayQueryBuilder::new()
        .year(2023)
        .week(1)
        .offense("michigan")
        .defense("ohio%20state")
        .build();
    let result = client.get_plays(&query).await.unwrap();

    let expected: Vec<Play> =
        serde_json::from_str(include_str!("common/fixtures/plays_offense_defense.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_plays_classification_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = PlayQueryBuilder::new()
        .year(2023)
        .week(1)
        .classification(DivisionClassification::Fcs)
        .build();
    let result = client.get_plays(&query).await.unwrap();

    let expected: Vec<Play> =
        serde_json::from_str(include_str!("common/fixtures/plays_classification.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_plays_season_type_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = PlayQueryBuilder::new()
        .year(2023)
        .week(1)
        .season_type(SeasonType::Postseason)
        .build();
    let result = client.get_plays(&query).await.unwrap();

    let expected: Vec<Play> =
        serde_json::from_str(include_str!("common/fixtures/plays_season_type.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_plays_conference_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = PlayQueryBuilder::new()
        .year(2023)
        .week(1)
        .conference("sec")
        .build();
    let result = client.get_plays(&query).await.unwrap();

    let expected: Vec<Play> =
        serde_json::from_str(include_str!("common/fixtures/plays_conference.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_plays_play_type_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = PlayQueryBuilder::new()
        .year(2023)
        .week(1)
        .play_type("RUSH")
        .build();
    let result = client.get_plays(&query).await.unwrap();

    let expected: Vec<Play> =
        serde_json::from_str(include_str!("common/fixtures/plays_play_type.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_plays_team_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = PlayQueryBuilder::new()
        .year(2023)
        .week(1)
        .team("auburn")
        .build();
    let result = client.get_plays(&query).await.unwrap();

    let expected: Vec<Play> =
        serde_json::from_str(include_str!("common/fixtures/plays_team.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_plays_required_year_week_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = PlayQueryBuilder::new().year(2023).week(1).build();
    let result = client.get_plays(&query).await.unwrap();

    let expected: Vec<Play> = serde_json::from_str(include_str!(
        "common/fixtures/plays_required_year_week.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}
