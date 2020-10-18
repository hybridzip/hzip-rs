pub trait SessionManager {
    fn refresh_session(address: String, key: String);
}