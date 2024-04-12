#![allow(dead_code)]
use super::Attributes;
use crate::public_api::model::{Collection, Object};

pub type VtFile = Object<Attributes>;
pub type VtFiles = Collection<Attributes>;
