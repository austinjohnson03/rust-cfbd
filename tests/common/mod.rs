use once_cell::sync::OnceCell;
use rust_cfbd::client::CfbdClient;
use rust_cfbd::config::Config;
use wiremock::MockServer;

pub use stubs::coaches::{
    mount_coaches_first_name_param_stub, mount_coaches_last_name_param_stub,
    mount_coaches_max_year_param_stub, mount_coaches_min_year_param_stub,
    mount_coaches_no_params_stub, mount_coaches_team_param_stub, mount_coaches_year_param_stub,
};
pub use stubs::conferences::mount_conference_no_params_stub;
pub use stubs::drives::{
    mount_drives_classification_param_stub, mount_drives_conference_param_stub,
    mount_drives_off_conference_def_conference_params_stub,
    mount_drives_offense_defense_param_stub, mount_drives_required_year_stub,
    mount_drives_season_type_param_stub, mount_drives_team_param_stub,
    mount_drives_week_param_stub,
};
pub use stubs::games::{
    mount_games_classification_param_stub, mount_games_conference_param_stub,
    mount_games_home_away_params_stub, mount_games_media_classification_param_stub,
    mount_games_media_conference_param_stub, mount_games_media_media_type_param_stub,
    mount_games_media_required_year_param_stub, mount_games_media_season_type_param_stub,
    mount_games_media_team_param_stub, mount_games_media_week_param_stub,
    mount_games_required_id_param_stub, mount_games_required_year_param_stub,
    mount_games_season_type_param_stub, mount_games_team_param_stub, mount_games_week_stub,
};
pub use stubs::info::mount_info_no_params_stub;
pub use stubs::plays::{
    mount_plays_classification_param_stub, mount_plays_conference_param_stub,
    mount_plays_off_conference_def_conference_params_stub, mount_plays_offense_defense_params_stub,
    mount_plays_play_type_param_stub, mount_plays_required_year_week_stub,
    mount_plays_season_type_param_stub, mount_plays_team_param_stub,
};
pub use stubs::rankings::{
    mount_rankings_required_year_param_stub, mount_rankings_season_type_param_stub,
    mount_rankings_week_param_stub,
};
pub use stubs::recruiting::{
    mount_recruiting_players_classification_param_stub,
    mount_recruiting_players_position_param_stub,
    mount_recruiting_players_required_team_param_stub,
    mount_recruiting_players_required_year_param_stub, mount_recruiting_players_state_param_stub,
    mount_recruiting_teams_no_params_stub, mount_recruiting_teams_team_param_stub,
    mount_recruiting_teams_year_param_stub,
};
pub use stubs::roster::{
    mount_teams_roster_classification_param_stub,
    mount_teams_roster_team_param_stub,
    mount_teams_roster_year_param_stub,
    mount_teams_roster_no_params_stub,
};

mod stubs;

static MOCK_SERVER: OnceCell<MockServer> = OnceCell::new();

pub async fn get_server() -> &'static MockServer {
    if let Some(s) = MOCK_SERVER.get() {
        return s;
    }

    let server = MockServer::start().await;

    // coaches
    mount_coaches_first_name_param_stub(&server).await;
    mount_coaches_last_name_param_stub(&server).await;
    mount_coaches_max_year_param_stub(&server).await;
    mount_coaches_min_year_param_stub(&server).await;
    mount_coaches_team_param_stub(&server).await;
    mount_coaches_year_param_stub(&server).await;
    mount_coaches_no_params_stub(&server).await;

    // conferences
    mount_conference_no_params_stub(&server).await;

    // drives
    mount_drives_off_conference_def_conference_params_stub(&server).await;
    mount_drives_offense_defense_param_stub(&server).await;
    mount_drives_classification_param_stub(&server).await;
    mount_drives_conference_param_stub(&server).await;
    mount_drives_season_type_param_stub(&server).await;
    mount_drives_team_param_stub(&server).await;
    mount_drives_week_param_stub(&server).await;
    mount_drives_required_year_stub(&server).await;

    // games
    mount_games_home_away_params_stub(&server).await;
    mount_games_season_type_param_stub(&server).await;
    mount_games_classification_param_stub(&server).await;
    mount_games_conference_param_stub(&server).await;
    mount_games_team_param_stub(&server).await;
    mount_games_week_stub(&server).await;
    mount_games_required_year_param_stub(&server).await;
    mount_games_required_id_param_stub(&server).await;

    // games/media
    mount_games_media_classification_param_stub(&server).await;
    mount_games_media_season_type_param_stub(&server).await;
    mount_games_media_conference_param_stub(&server).await;
    mount_games_media_media_type_param_stub(&server).await;
    mount_games_media_team_param_stub(&server).await;
    mount_games_media_week_param_stub(&server).await;
    mount_games_media_required_year_param_stub(&server).await;

    // info
    mount_info_no_params_stub(&server).await;

    // plays
    mount_plays_off_conference_def_conference_params_stub(&server).await;
    mount_plays_offense_defense_params_stub(&server).await;
    mount_plays_classification_param_stub(&server).await;
    mount_plays_season_type_param_stub(&server).await;
    mount_plays_conference_param_stub(&server).await;
    mount_plays_play_type_param_stub(&server).await;
    mount_plays_team_param_stub(&server).await;
    mount_plays_required_year_week_stub(&server).await;

    // rankings
    mount_rankings_season_type_param_stub(&server).await;
    mount_rankings_week_param_stub(&server).await;
    mount_rankings_required_year_param_stub(&server).await;

    // recruiting/players
    mount_recruiting_players_classification_param_stub(&server).await;
    mount_recruiting_players_position_param_stub(&server).await;
    mount_recruiting_players_state_param_stub(&server).await;
    mount_recruiting_players_required_year_param_stub(&server).await;
    mount_recruiting_players_required_team_param_stub(&server).await;

    // recruiting/teams
    mount_recruiting_teams_team_param_stub(&server).await;
    mount_recruiting_teams_year_param_stub(&server).await;
    mount_recruiting_teams_no_params_stub(&server).await;
    
    // roster
    mount_teams_roster_classification_param_stub(&server).await;
    mount_teams_roster_year_param_stub(&server).await;
    mount_teams_roster_team_param_stub(&server).await;
    mount_teams_roster_no_params_stub(&server).await;

    MOCK_SERVER.get_or_init(|| server)
}

pub fn mock_client(server: &MockServer) -> CfbdClient {
    let config = Config::new("MOCK_KEY").with_host_url(server.uri());
    CfbdClient::new(config)
}
