use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SessionUser {
    pub user_id: Uuid,
}

const SESSION_COOKIE_NAME: &str = "session_user_id";

pub fn create_session_cookie(user_id: Uuid) -> Cookie<'static> {
    let mut cookie = Cookie::new(SESSION_COOKIE_NAME, user_id.to_string());
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie
}

pub fn clear_session_cookie() -> Cookie<'static> {
    let mut cookie = Cookie::new(SESSION_COOKIE_NAME, "");
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.make_removal();
    cookie
}

pub fn read_session_user(jar: &CookieJar) -> Option<SessionUser> {
    let cookie = jar.get(SESSION_COOKIE_NAME)?;
    let user_id = Uuid::parse_str(cookie.value()).ok()?;
    Some(SessionUser { user_id })
}
