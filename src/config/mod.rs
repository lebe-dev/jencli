use std::fmt::{Display, Formatter};

use anyhow::Context;
use serde::Deserialize;

pub mod file;

#[derive(Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub jenkins_url: String,
    pub username: String,
    pub token: String,

    pub list: ListCommandConfig
}

#[derive(Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ListCommandConfig {
    pub exclude: Vec<String>
}

impl Display for AppConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[AppConfig] jenkins_url: '{}', username: '{}', token: '*******', list: {} [/AppConfig]",
               self.jenkins_url, self.username, self.list)
    }
}

impl Display for ListCommandConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "exclude: {:?}", self.exclude)
    }
}