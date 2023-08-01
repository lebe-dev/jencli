use std::fs;
use std::path::Path;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::jenkins::list::JenkinsJob;

const CACHE_DIR: &str = "cache";
const CACHE_FILE: &str = "jobs.cache";

#[derive(Serialize,Deserialize)]
pub struct JobListCache {
    pub jobs: Vec<JenkinsJob>
}

pub fn save_job_list_in_cache(jobs: &Vec<JenkinsJob>) -> anyhow::Result<()> {
    debug!("save job list into cache: {:?}", jobs);
    let cache_path = Path::new(CACHE_DIR);

    if !cache_path.exists() {
        fs::create_dir(cache_path)?;
    }

    let json = serde_json::to_string(&jobs)?;

    let cache_file_path = cache_path.join(CACHE_FILE);

    fs::write(cache_file_path, json)?;

    Ok(())
}

pub fn load_job_list_from_cache() -> anyhow::Result<Vec<JenkinsJob>> {
    let cache_file_path = Path::new(CACHE_DIR).join(CACHE_FILE);

    if cache_file_path.exists() {
        let json = fs::read_to_string(cache_file_path)?;

        let jobs = serde_json::from_str(&json)?;
        debug!("job list loaded from cache: {:?}", jobs);

        Ok(jobs)

    } else {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use fake::{Fake, Faker};
    use crate::cache::{CACHE_DIR, load_job_list_from_cache, save_job_list_in_cache};
    use crate::jenkins::list::JenkinsJob;

    #[test]
    fn cache_should_be_saved_and_loaded() {
        cleanup();

        let job1 = get_sample_jenkins_job();
        let job2 = get_sample_jenkins_job();

        save_job_list_in_cache(&vec![job1.clone(), job2.clone()]).unwrap();

        let results = load_job_list_from_cache().unwrap();

        assert_eq!(2, results.len());

        assert!(results.iter().find(|j|j.name == job1.name).is_some());
        assert!(results.iter().find(|j|j.name == job2.name).is_some());
    }

    fn get_sample_jenkins_job() -> JenkinsJob {
        JenkinsJob {
            name: get_random_string(),
            url: get_random_string(),
        }
    }

    fn cleanup() {
        let cache_path = Path::new(CACHE_DIR);

        if cache_path.exists() {
            fs::remove_dir(&cache_path).unwrap();
        }
    }

    fn get_random_string() -> String {
        Faker.fake::<String>()
    }
}