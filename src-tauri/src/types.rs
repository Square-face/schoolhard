use schoolsoft::types::SchoolListing;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

pub type Schools = Vec<School>;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
#[typeshare]
pub struct School {
    name: String,
    url: String,
    supported: bool,
}

impl From<SchoolListing> for School {
    fn from(school: SchoolListing) -> Self {
        School {
            name: school.name,
            url: school.url,
            supported: school.login_methods.student.contains(&4),
        }
    }
}
