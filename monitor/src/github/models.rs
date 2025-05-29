#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PullRequest {
    pub id: u64,
    pub title: String,
    commit_url: String,
}
