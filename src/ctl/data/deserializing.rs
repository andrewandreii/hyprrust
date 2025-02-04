use super::data_models::{FullscreenState, Layer, Levels, Sides};
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

impl<'de> Deserialize<'de> for FullscreenState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use FullscreenState::*;

        match <u8 as Deserialize>::deserialize(deserializer) {
            Ok(0) => {
                return Ok(None);
            }
            Ok(1) => {
                return Ok(Maximized);
            }
            Ok(2) => {
                return Ok(Fullscreen);
            }
            Ok(3) => {
                return Ok(Max);
            }
            _ => {
                return Err(Error::custom("Invalid fullscreen value"));
            }
        };
    }
}

pub trait HyprlandData {
    fn get_command() -> &'static str;
}

pub trait HyprlandDataWithArgument {
    fn get_command(arg: String) -> String;
}

struct SidesVisitor;

impl SidesVisitor {
    fn new() -> Self {
        SidesVisitor {}
    }
}

impl<'de> SidesVisitor {
    fn next_element<A>(&self, seq: &mut A) -> Result<i32, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        if let Some(value) = seq.next_element()? {
            Ok(value)
        } else {
            Err(Error::custom("Expected int"))
        }
    }
}

impl<'de> Visitor<'de> for SidesVisitor {
    type Value = Sides;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an array of 4 i32")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let sides = Sides {
            left: self.next_element(&mut seq)?,
            top: self.next_element(&mut seq)?,
            right: self.next_element(&mut seq)?,
            bottom: self.next_element(&mut seq)?,
        };

        Ok(sides)
    }
}

impl<'de> Deserialize<'de> for Sides {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(4, SidesVisitor::new())
    }
}

impl<'de> Deserialize<'de> for Levels {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut levels_wrapper =
            <HashMap<String, HashMap<String, Vec<Layer>>>>::deserialize(deserializer)?;
        if let Some(levels) = levels_wrapper.remove("levels") {
            Ok(Levels(levels))
        } else {
            Err(Error::custom("Invalid structure"))
        }
    }
}
