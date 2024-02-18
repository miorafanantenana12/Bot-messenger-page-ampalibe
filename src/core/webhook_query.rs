use std::env;

use rocket::http::Status;
use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

#[derive(Debug)]
pub enum WebHookError<'a> {
    VerificationFailed(&'a str),
    ArgsNotEnough,
}

pub struct WebHookQuery {
    pub hub_challenge: String,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for WebHookQuery {
    type Error = WebHookError<'a>;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let query = request
            .uri()
            .query()
            .expect("failed to get query from request");

        let args: Vec<_> = query.segments().collect();

        let mut hub_mode = None;
        let mut hub_challenge = None;
        let mut token = None;

        for (key, value) in args {
            match key {
                "hub.mode" => hub_mode = Some(value),
                "hub.challenge" => hub_challenge = Some(value),
                "hub.verify_token" => token = Some(value),
                _ => (),
            }
        }

        match (hub_mode, hub_challenge, token) {
            (Some(hub_mode), Some(hub_challenge), Some(token)) => {
                if hub_mode.eq("subscribe") && env::var("VERIFY_TOKEN").unwrap().eq(token) {
                    Outcome::Success(Self {
                        hub_challenge: hub_challenge.to_string(),
                    })
                } else {
                    Outcome::Error((
                        Status::Unauthorized,
                        WebHookError::VerificationFailed("Token mismatch"),
                    ))
                }
            }
            _ => Outcome::Error((Status::Unauthorized, WebHookError::ArgsNotEnough)),
        }
    }
}
