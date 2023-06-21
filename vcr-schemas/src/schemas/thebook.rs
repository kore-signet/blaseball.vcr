
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
pub struct Thebook {
    pub collection: Vec<Collection>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Collection {
    pub footer: Footer,

    pub header: Header,

    pub id: String,

    pub sections: Vec<Section>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Footer {
    pub header: String,

    pub label: String,

    pub text: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Header {
    pub aria: String,

    pub title: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Section {
    pub bullet: String,

    pub subbullets: Vec<String>,
}
