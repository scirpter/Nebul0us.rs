pub enum ConnectionState {
    DISCONNECTED,
    CONNECTING,
    CONNECTED,
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState::DISCONNECTED
    }
}
