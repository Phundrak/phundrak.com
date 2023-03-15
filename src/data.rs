use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub user: User,
}

/// GitHub user
///
/// Contains their newest, most starred, and pinned repositories,
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub newest: Newest,
    pub most_starred: MostStarred,
    pub pinned: Pinned,
}

/// Newest repositories
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Newest {
    pub nodes: Vec<Repository>,
}

/// Most starred repositories
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MostStarred {
    pub nodes: Vec<Repository>,
}

/// Pinned repositories
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pinned {
    pub nodes: Vec<Repository>,
}

/// Repository
///
/// Contains the name of the repository, the amount of stars, and the
/// amount of forks of the repository
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "node")]
pub struct Repository {
    pub name: String,
    pub stargazer_count: i64,
    pub fork_count: i64,
}
