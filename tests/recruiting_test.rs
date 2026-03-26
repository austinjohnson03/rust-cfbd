use rust_cfbd::models::cfb::entity::recruit::{Recruit, RecruitClassification};
use rust_cfbd::models::cfb::entity::team_recruiting_ranking::TeamRecruitingRanking;
use rust_cfbd::models::cfb::query::recruit_player_query::RecruitPlayerQueryBuilder;
use rust_cfbd::models::cfb::query::recruit_team_query::RecruitTeamQueryBuilder;

pub mod common;

#[tokio::test]
async fn test_recruiting_players_classification_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = RecruitPlayerQueryBuilder::new()
        .year(2022)
        .classification(RecruitClassification::HighSchool)
        .build();
    let result = client.get_recruiting_players(&query).await.unwrap();

    let expected: Vec<Recruit> = serde_json::from_str(include_str!(
        "common/fixtures/recruiting_players_classification.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_recruiting_player_position_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = RecruitPlayerQueryBuilder::new()
        .year(2022)
        .position("qb")
        .build();
    let result = client.get_recruiting_players(&query).await.unwrap();

    let expected: Vec<Recruit> = serde_json::from_str(include_str!(
        "common/fixtures/recruiting_players_position.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_recruiting_player_state_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = RecruitPlayerQueryBuilder::new()
        .year(2022)
        .state("al")
        .build();
    let result = client.get_recruiting_players(&query).await.unwrap();

    let expected: Vec<Recruit> = serde_json::from_str(include_str!(
        "common/fixtures/recruiting_players_state.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_recruiting_player_required_year_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = RecruitPlayerQueryBuilder::new().year(2022).build();
    let result = client.get_recruiting_players(&query).await.unwrap();

    let expected: Vec<Recruit> = serde_json::from_str(include_str!(
        "common/fixtures/recruiting_players_required_year.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_recruiting_player_required_team_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = RecruitPlayerQueryBuilder::new().team("alabama").build();
    let result = client.get_recruiting_players(&query).await.unwrap();

    let expected: Vec<Recruit> = serde_json::from_str(include_str!(
        "common/fixtures/recruiting_players_required_team.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_recruiting_team_team_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = RecruitTeamQueryBuilder::new().team("clemson").build();
    let result = client.get_recruiting_teams(&query).await.unwrap();

    let expected: Vec<TeamRecruitingRanking> =
        serde_json::from_str(include_str!("common/fixtures/recruiting_teams_team.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_recruiting_team_year_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = RecruitTeamQueryBuilder::new().year(2023).build();
    let result = client.get_recruiting_teams(&query).await.unwrap();

    let expected: Vec<TeamRecruitingRanking> =
        serde_json::from_str(include_str!("common/fixtures/recruiting_teams_year.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_recruiting_team_no_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = RecruitTeamQueryBuilder::new().build();
    let result = client.get_recruiting_teams(&query).await.unwrap();

    let expected: Vec<TeamRecruitingRanking> = serde_json::from_str(include_str!(
        "common/fixtures/recruiting_teams_no_params.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}
