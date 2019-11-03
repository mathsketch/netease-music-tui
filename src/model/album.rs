#[allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use super::artist::Artist;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Album {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub size: Option<i32>,
    pub artist: Option<Artist>,
}