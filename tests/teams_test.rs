use rust_cfbd::models::cfb::entity::team::Team;
use rust_cfbd::models::cfb::query::team_query::TeamQueryBuilder;

mod common;

#[tokio::test]
async fn test_team_conference_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = TeamQueryBuilder::new().conference("sec").build();
    let result = client.get_teams(&query).await.unwrap();

    let expected: Vec<Team> =
        serde_json::from_str(include_str!("common/fixtures/teams_conference.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_team_year_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = TeamQueryBuilder::new().year(2024).build();
    let result = client.get_teams(&query).await.unwrap();

    let expected: Vec<Team> =
        serde_json::from_str(include_str!("common/fixtures/teams_year.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_team_no_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = TeamQueryBuilder::new().build();
    let result = client.get_teams(&query).await.unwrap();

    let expected: Vec<Team> =
        serde_json::from_str(include_str!("common/fixtures/teams_no_params.json")).unwrap();

    assert_eq!(result, expected);
}
