#[derive(Default, Clone)]
pub enum ConnectionState {
    #[default]
    DISCONNECTED,
    CONNECTING,
    CONNECTED,
}
