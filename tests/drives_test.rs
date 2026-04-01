use rust_cfbd::models::cfb::{
    entity::{
        division_classification::DivisionClassification, drive::Drive, season_type::SeasonType,
    },
    query::drive_query::DriveQueryBuilder,
};

mod common;

#[tokio::test]
async fn test_drives_off_conference_def_conference_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = DriveQueryBuilder::new()
        .year(2025)
        .offense_conference("acc")
        .defense_conference("sec")
        .build();
    let result = client.get_drives(&query).await.unwrap();

    let expected: Vec<Drive> = serde_json::from_str(include_str!(
        "common/fixtures/drives_offense_conference_defense_conference.json"
    ))
    .unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_drives_offense_defense_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = DriveQueryBuilder::new()
        .year(2023)
        .offense("ohio%20state")
        .defense("michigan")
        .build();
    let result = client.get_drives(&query).await.unwrap();

    let expected: Vec<Drive> =
        serde_json::from_str(include_str!("common/fixtures/drives_offense_defense.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_drives_classification_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = DriveQueryBuilder::new()
        .year(2023)
        .classification(DivisionClassification::Fbs)
        .build();
    let result = client.get_drives(&query).await.unwrap();

    let expected: Vec<Drive> =
        serde_json::from_str(include_str!("common/fixtures/drives_classification.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_drives_conference_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = DriveQueryBuilder::new()
        .year(2023)
        .conference("sec")
        .build();
    let result = client.get_drives(&query).await.unwrap();

    let expected: Vec<Drive> =
        serde_json::from_str(include_str!("common/fixtures/drives_conference.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_drives_season_type_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = DriveQueryBuilder::new()
        .year(2023)
        .season_type(SeasonType::Postseason)
        .build();
    let result = client.get_drives(&query).await.unwrap();

    let expected: Vec<Drive> =
        serde_json::from_str(include_str!("common/fixtures/drives_season_type.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_drives_team_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = DriveQueryBuilder::new().year(2023).team("texas").build();
    let result = client.get_drives(&query).await.unwrap();

    let expected: Vec<Drive> =
        serde_json::from_str(include_str!("common/fixtures/drives_team.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_drives_week_params() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = DriveQueryBuilder::new().year(2023).week(1).build();
    let result = client.get_drives(&query).await.unwrap();

    let expected: Vec<Drive> =
        serde_json::from_str(include_str!("common/fixtures/drives_week.json")).unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_drives_required_year_param() {
    let server = common::get_server().await;
    let client = common::mock_client(server);
    let query = DriveQueryBuilder::new().year(2023).build();
    let result = client.get_drives(&query).await.unwrap();

    let expected: Vec<Drive> =
        serde_json::from_str(include_str!("common/fixtures/drives_required_year.json")).unwrap();

    assert_eq!(result, expected);
}
