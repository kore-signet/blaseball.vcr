
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
pub struct Thebeat {
    pub collection: Vec<Collection>,

    pub recap: Recap,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Collection {
    pub contents: Contents,

    pub date: String,

    pub id: String,

    pub title: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    pub articles: Vec<Article>,

    pub blaseball_link: String,

    pub closing: String,

    pub intro: String,

    pub special_headlines: Vec<SpecialHeadline>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Article {
    pub article: String,

    pub heading: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SpecialHeadline {
    pub color: Option<String>,

    pub heading: Option<String>,

    pub subheading: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Recap {
    pub beat: String,

    pub content: Vec<Content>,

    pub deeper_content: Vec<DeeperContent>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Content {
    pub header: String,

    pub text: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct DeeperContent {
    pub header: String,

    pub text: Vec<String>,
}
