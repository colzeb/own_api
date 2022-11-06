use actix_web::{guard::{Guard, GuardContext}, http};

pub struct LoginGuard;

impl Guard for LoginGuard{
    fn check(&self, req: &GuardContext) -> bool {
        req.head()
            .headers()
            .contains_key(http::header::CONTENT_TYPE)
    }
}