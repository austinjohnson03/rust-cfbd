use rust_cfbd::models::cfb::entity::division_classification::DivisionClassification;
use rust_cfbd::models::cfb::entity::roster_player::RosterPlayer;
use rust_cfbd::models::cfb::query::roster_query::RosterQueryBuilder;

mod common;

#[tokio::test]
async fn test_teams_roster_classification_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = RosterQueryBuilder::new()
        .classification(DivisionClassification::Fbs)
        .build();
    let result = client.get_roster(&query).await.unwrap();

    let expected: Vec<RosterPlayer> =
        serde_json::from_str(include_str!("common/fixtures/roster_classification.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_teams_roster_year_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = RosterQueryBuilder::new().year(2022).build();
    let result = client.get_roster(&query).await.unwrap();

    let expected: Vec<RosterPlayer> =
        serde_json::from_str(include_str!("common/fixtures/roster_year.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_teams_roster_team_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = RosterQueryBuilder::new().team("florida").build();
    let result = client.get_roster(&query).await.unwrap();

    let expected: Vec<RosterPlayer> =
        serde_json::from_str(include_str!("common/fixtures/roster_team.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_teams_roster_no_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = RosterQueryBuilder::new().build();
    let result = client.get_roster(&query).await.unwrap();

    let expected: Vec<RosterPlayer> =
        serde_json::from_str(include_str!("common/fixtures/roster_no_params.json")).unwrap();

    assert_eq!(result, expected);
}
