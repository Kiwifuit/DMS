use std::ops::Deref;

use serde::Deserialize;

mod project;
mod query;

#[derive(Debug)]
#[repr(transparent)]
pub struct CurseResponse<T> {
  inner: T,
}

impl<T> Deref for CurseResponse<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl<T> CurseResponse<T> {
  pub fn new(inner: T) -> Self {
    Self { inner }
  }
}

impl<'de, T> Deserialize<'de> for CurseResponse<T>
where
  T: Deserialize<'de>,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let data = serde_json::Map::deserialize(deserializer)?
      .remove("data")
      .ok_or_else(|| serde::de::Error::missing_field("data"))?;

    let data: T = T::deserialize(data).map_err(serde::de::Error::custom)?;
    Ok(Self { inner: data })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde::Deserialize;

  #[derive(Debug, Deserialize)]
  struct TestStruct {
    name: String,
  }

  #[test]
  fn test_cresp() {
    let json_data = r#"
      {
        "data": {
          "name": "Samuel L Jackson"
        }
      }
    "#;

    let data = serde_json::from_str::<CurseResponse<TestStruct>>(&json_data);

    assert!(data.is_ok());
    assert_eq!(data.unwrap().name, "Samuel L Jackson".to_string());
  }
}
