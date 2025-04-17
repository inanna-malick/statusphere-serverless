///The askama template types for HTML
///
use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'a> {
    pub title: &'a str,
    pub status_options: &'a [&'a str],
    pub profile: Option<Profile>,
    pub my_status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub did: String,
    pub display_name: Option<String>,
}