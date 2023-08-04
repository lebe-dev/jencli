use std::collections::HashMap;

use anyhow::anyhow;
use log::info;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::{Deserialize, Deserializer, Serialize};
use urlencoding::encode;

use crate::jenkins::auth::{get_crumb_issuer, JenkinsCrumbIssuer};

pub fn rebuild_job(client: &Client, jenkins_url: &str, username: &str, token: &str, job_name: &str) -> anyhow::Result<()> {
    info!("attempt to rebuild job '{job_name}' at '{jenkins_url}'..");

    info!("getting latest build information..");

    let build_info = get_job_build_info(&client, &jenkins_url,
                                        &username, &token, &job_name)?;

    let crumb_issuer = get_crumb_issuer(&client, &jenkins_url,
                                        &username, &token)?;

    let url_params = get_url_params(&build_info, &crumb_issuer)?;
    let url_params = encode(&url_params);
    let url_params = format!("{url_params}");

    let url = format!("https://jenkins.sk.ru/job/{job_name}/buildWithParameters?{}", url_params);

    info!("url '{url}'");

    info!("executing rebuild for job '{job_name}'..");
    let resp = client.post(url).basic_auth(&username, Some(token)).send()?;

    let status = resp.status();

    info!("server response: {}", status);

    if status == StatusCode::CREATED {
        info!("rebuild for job '{job_name}' successfully executed");
        Ok(())

    } else {
        Err(anyhow!("unexpected server response"))
    }
}

fn get_job_build_info(client: &Client, jenkins_url: &str, username: &str,
                      token: &str, job_name: &str) -> anyhow::Result<JenkinsBuildInfo> {
    let url = format!("{jenkins_url}/job/{job_name}/lastBuild/api/json");

    let resp = client.get(&url).basic_auth(&username, Some(token)).send()?;

    let build_info = resp.json::<JenkinsBuildInfo>()?;

    info!("build info: {:?}", build_info);

    Ok(build_info)
}

/// name=SCM_BRANCH&value=dev&name=CLEAR_DOCKER_CACHE&name=UPDATE_DB_RIGHTS&Jenkins-Crumb=0e1dcbfe09b0ceb2c47b95c0df172b7a0d976b49782a75adbed124ef526df6a1&json=%7B%22parameter%22%3A+%5B%7B%22name%22%3A+%22SCM_BRANCH%22%2C+%22va    lue%22%3A+%22dev%22%7D%2C+%7B%22name%22%3A+%22CLEAR_DOCKER_CACHE%22%2C+%22value%22%3A+false%7D%2C+%7B%22name%22%3A+%22UPDATE_DB_RIGHTS%22%2C+%22value%22%3A+false%7D%5D%2C+%22Jenkins-Crumb%22%3A+%220e1dcbfe09b0ceb2c47b95c0df1    72b7a0d976b49782a75adbed124ef526df6a1%22%7D&Submit=Rebuild
fn get_url_params(build_info: &JenkinsBuildInfo,
                  crumb_issuer: &JenkinsCrumbIssuer) -> anyhow::Result<String> {

    let mut result: Vec<String> = vec![];

    if let Some(action) = build_info.actions.iter().find(|a| a.parameters.is_some()) {
        let params = action.parameters.clone().unwrap();

        let mut json_props: Vec<HashMap<String, String>> = vec![];

        params.iter().for_each(|p| {
            result.push(format!("name={}", p.name));
            result.push(format!("value={}", p.value));

            json_props.push(HashMap::from([(p.name.to_string(), p.value.to_string())]))
        });

        result.push(format!("{}={}", crumb_issuer.crumb_request_field, crumb_issuer.crumb));

        let rebuild_url_params = JenkinsRebuildUrlParams {
            parameter: json_props.clone()
        };

        let mut json_param = serde_json::to_string(&rebuild_url_params)?;

        json_param = json_param.replace("\"true\"", "true")
                               .replace("\"false\"", "false");

        result.push(format!("json={json_param}"));
    }

    let mut result = result.join("&").to_string();

    if result.ends_with("&") {
        result.pop();
    }

    Ok(result.to_string())
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct JenkinsRebuildUrlParams {
    pub parameter: Vec<HashMap<String, String>>
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct JenkinsBuildInfo {
    pub number: u32,
    pub url: String,
    pub actions: Vec<JenkinsBuildAction>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct JenkinsBuildAction {
    pub parameters: Option<Vec<JenkinsBuildParam>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct JenkinsBuildParam {
    pub name: String,
    #[serde(deserialize_with = "str_or_bool")]
    pub value: String,
}

#[derive(Deserialize, Debug, Clone)]
struct JenkinsBuildParamValue;

fn str_or_bool<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrOrBool<'a> {
        Str(&'a str),
        Bool(bool),
    }

    Ok(match StrOrBool::deserialize(deserializer)? {
        StrOrBool::Str(v) => v.parse().unwrap_or("".to_string()),
        StrOrBool::Bool(v) => v.to_string(),
    })
}

#[cfg(test)]
mod params_deserialization_tests {
    use crate::jenkins::build::JenkinsBuildParam;

    #[test]
    fn test_for_boolean() {
        let input = r#"{
            "name":"charlie",
            "value": true
        }"#;

        let result = serde_json::from_str::<JenkinsBuildParam>(input).unwrap();

        assert_eq!(result.name, "charlie".to_string());
        assert_eq!(result.value, "true".to_string());
    }

    #[test]
    fn test_for_string() {
        let input = r#"{
            "name":"robin",
            "value":"something"
        }"#;

        let result = serde_json::from_str::<JenkinsBuildParam>(input).unwrap();

        assert_eq!(result.name, "robin".to_string());
        assert_eq!(result.value, "something".to_string());
    }
}