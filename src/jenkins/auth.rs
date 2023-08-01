use log::info;
use reqwest::blocking::Client;
use serde::Deserialize;

/// https://stackoverflow.com/a/54750559/1511077
///
/// ```json
/// {
//     "_class": "hudson.security.csrf.DefaultCrumbIssuer",
//     "crumb": "da429603cefa8bdf8a5c9791c51e18daa020a515009fcfce2cc95121629e43fd",
//     "crumbRequestField": "Jenkins-Crumb"
//  }
/// ```
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JenkinsCrumbIssuer {
    pub crumb: String,
    pub crumb_request_field: String,
}

pub fn get_crumb_issuer(client: &Client, jenkins_url: &str,
                        username: &str, token: &str) -> anyhow::Result<JenkinsCrumbIssuer> {
    info!("get crumb issuer from '{jenkins_url}'..");

    let url = format!("{jenkins_url}/crumbIssuer/api/json");

    let resp = client.get(url).basic_auth(username, Some(token)).send()?;

    let issuer = resp.json::<JenkinsCrumbIssuer>()?;

    info!("issuer received: {:?}", issuer);

    Ok(issuer)
}