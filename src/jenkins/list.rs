use log::info;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use crate::cache::{load_job_list_from_cache, save_job_list_in_cache};

#[derive(Serialize,Deserialize)]
pub struct JenkinsApiResponse {
    pub jobs: Vec<JenkinsJob>
}

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct JenkinsJob {
    pub name: String,
    pub url: String
}

pub fn get_jenkins_job_list(client: &Client, jenkins_url: &str,
                            username: &str, token: &str) -> anyhow::Result<Vec<JenkinsJob>> {
    info!("get job list from jenkins '{jenkins_url}', username '{username}'");

    let job_list_from_cache = load_job_list_from_cache()?;

    if job_list_from_cache.is_empty() {
        let url = format!("{jenkins_url}/api/json");

        let resp = client.get(&url).basic_auth(username, Some(token)).send()?;

        let resp = resp.json::<JenkinsApiResponse>()?;

        info!("jobs:");
        info!("{:?}", resp.jobs);

        save_job_list_in_cache(&resp.jobs)?;

        Ok(resp.jobs.clone())

    } else {
        info!("jobs: (from cache)");
        info!("{:?}", job_list_from_cache);

        Ok(job_list_from_cache)
    }
}

#[cfg(test)]
mod tests {
    use reqwest::blocking::ClientBuilder;
    use crate::jenkins::list::get_jenkins_job_list;

    #[ignore]
    #[test]
    fn job_list_should_be_returned() {
        let jenkins_url = "CHANGE-ME";
        let username = "CHANGE-ME";
        let token = "CHANGE-ME";

        let client = ClientBuilder::new().build().unwrap();

        let jobs = get_jenkins_job_list(&client, &jenkins_url, &username, &token).unwrap();

        assert!(!jobs.is_empty())
    }
}