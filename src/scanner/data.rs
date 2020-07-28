use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ChildResult {
    title: String,
    permalink: String,
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
