use serde::{ser::SerializeSeq, Serialize};

use super::project::CurseMod;

#[derive(Debug)]
pub struct CurseMods {
  mods: Vec<CurseMod>,
}

impl Serialize for CurseMods {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.mods.len()))?;

    for mod_data in &self.mods {
      seq.serialize_element(&mod_data.id)?;
    }

    seq.end()
  }
}

impl From<Vec<CurseMod>> for CurseMods {
  fn from(value: Vec<CurseMod>) -> Self {
    Self { mods: value }
  }
}

// Idea: Might want to add a "growable"
// API. Where you start with an empty
// CurseMods struct and iteratively
// fill it or something
//
// Something like this:
// impl CurseMods {
//   pub fn new() -> Self {
//     Self {
//       mods: vec![]
//     }
//   }

//   pub fn add_mod(&mut self, new_mod: CurseMod) {
//     self.mods.push(new_mod);
//   }
// }
