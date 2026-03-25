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

    MOCK_SERVER.get_or_init(|| server)
}

pub fn mock_client(server: &MockServer) -> CfbdClient {
    let config = Config::new("MOCK_KEY").with_host_url(server.uri());
    CfbdClient::new(config)
}
