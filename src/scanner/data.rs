use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ChildResult {
    pub url: String,
    pub id: String,
    pub title: String,
    pub thumbnail: String,
}

#[derive(Deserialize, Debug)]
struct Child {
    data: ChildResult,
}

#[derive(Deserialize, Debug)]
struct Children {
    children: Vec<Child>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SearchResult {
    data: Children,
}

impl SearchResult {
    pub fn get_latest_result(&self) -> Option<ChildResult> {
        self.data.children.first().map(|child| child.data.clone())
    }
}
