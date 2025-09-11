use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
use time::{Duration, OffsetDateTime};

use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

pub static SESSION_STORE: Lazy<Arc<Mutex<SessionStore>>> =
    Lazy::new(|| Arc::new(Mutex::new(SessionStore::new(3600))));

#[derive(Debug)]
pub struct SessionData {
    pub _created_at: OffsetDateTime,
    pub last_seen: OffsetDateTime,
    // Optionally:
    // pub visit_count: u32,
}
#[derive(Debug)]
pub struct SessionStore {
    sessions: HashMap<String, SessionData>,
    timeout: Duration,
}

impl SessionStore {
    pub fn new(timeout_secs: i64) -> Self {
        Self {
            sessions: HashMap::new(),
            timeout: Duration::seconds(timeout_secs),
        }
    }

    pub fn create_session(&mut self) -> String {
        let session_id = generate_session_id();
        let now = OffsetDateTime::now_utc();
        let data = SessionData {
            _created_at: now,
            last_seen: now,
        };
        self.sessions.insert(session_id.clone(), data);
        session_id
    }

    pub fn get_session(&mut self, session_id: &str) -> Option<&mut SessionData> {
        self.remove_expired();
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.last_seen = OffsetDateTime::now_utc();
            Some(session)
        } else {
            None
        }
    }

    pub fn remove_expired(&mut self) {
        let now = OffsetDateTime::now_utc();
        self.sessions
            .retain(|_, data| data.last_seen + self.timeout > now);
    }
}

pub fn generate_session_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

pub fn build_set_cookie_header(session_id: &str) -> String {
    format!(
        "Set-Cookie: session_id={}; HttpOnly; Path=/; Max-Age=3600", // not secure cause of HTTP (not HTTPS)
        session_id
    )
}

pub fn parse_cookie_header(header: &str) -> HashMap<String, String> {
    header
        .split(';')
        .filter_map(|cookie| {
            let mut parts = cookie.trim().splitn(2, '=');
            match (parts.next(), parts.next()) {
                (Some(k), Some(v)) => Some((k.to_string(), v.to_string())),
                _ => None,
            }
        })
        .collect()
}
