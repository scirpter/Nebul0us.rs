#[derive(Default)]
pub enum ConnectionState {
    #[default]
    DISCONNECTED,
    CONNECTING,
    CONNECTED,
}
