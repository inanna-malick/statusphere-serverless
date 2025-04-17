///The askama template types for HTML
///
use askama::Template;
use axum::response::{Html, IntoResponse};
use serde::{Deserialize, Serialize};

use super::lexicons::xyz::statusphere::status;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub status_options: &'static [&'static str],
    pub profile: Option<Profile>,
    pub my_status: Option<status::RecordData>,
}

impl IntoResponse for HomeTemplate {
    fn into_response(self) -> axum::response::Response {
        let html = self.render().expect("template should be valid");

        Html::from(html).into_response()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub did: String,
    pub display_name: Option<String>,
}
