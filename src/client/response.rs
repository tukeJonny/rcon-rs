/// Enumerates the possible results for srcds authentication.
pub enum AuthResponse {
    /// This enumerate describes SERVERDATA_AUTH succeeded
    AuthenticationSucceeded,
    /// This enumerate describes SERVERDATA_AUTH failed
    AuthenticationFailed,
}
