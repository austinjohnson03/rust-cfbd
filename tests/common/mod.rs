use once_cell::sync::OnceCell;
use rust_cfbd::client::CfbdClient;
use rust_cfbd::config::Config;
use wiremock::MockServer;

mod stubs;

static MOCK_SERVER: OnceCell<MockServer> = OnceCell::new();

pub async fn get_server() -> &'static MockServer {
    if let Some(s) = MOCK_SERVER.get() {
        return s;
    }

    let server = MockServer::start().await;
    MOCK_SERVER.get_or_init(|| server)
}

pub fn mock_client(server: &MockServer) -> CfbdClient {
    let config = Config::new("MOCK_KEY").with_host_url(server.uri());
    CfbdClient::new(config)
}
