use std::rc::Rc;

use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct CurseMod {
  pub(crate) id: u32,
  pub name: Rc<str>,
  pub links: CurseModLinks,
  pub summary: Rc<str>,

  #[serde(
    rename = "primaryCategoryId",
    deserialize_with = "CurseCategory::from_id"
  )]
  pub primary_category: CurseCategory,
  pub categories: Rc<[CurseCategory]>,

  pub authors: Rc<[CurseAuthor]>,
  pub logo: CurseAsset,
  pub screenshots: Rc<[CurseAsset]>,

  #[serde(rename = "allowModDistribution")]
  allowed: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CurseModLinks {
  #[serde(rename = "websiteUrl")]
  pub website: Rc<str>,
  #[serde(rename = "wikiUrl")]
  pub wiki: Rc<str>,
  #[serde(rename = "issuesUrl")]
  pub issues: Rc<str>,
  #[serde(rename = "sourceUrl")]
  pub source: Rc<str>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]

pub struct CurseCategory {
  pub id: u32,
  pub name: Rc<str>,
  pub url: Rc<str>,
  pub icon_url: Rc<str>,
}

impl CurseCategory {
  fn from_id<'de, D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let id = u32::deserialize(deserializer)?;

    Ok(Self {
      id,
      name: Rc::from("test"),
      url: Rc::from("test"),
      icon_url: Rc::from("test"),
    })
  }
}

#[derive(Debug, Deserialize)]
pub struct CurseAuthor {
  pub id: u32,
  pub name: Rc<str>,
  pub url: Rc<str>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct CurseAsset {
  pub id: u32,
  pub title: Rc<str>,
  pub description: Rc<str>,
  pub thumbnail_url: Rc<str>,
  pub url: Rc<str>,
}
