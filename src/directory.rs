//! The module defines directory data structures

use convert::FromMap;
use error::{Error, ProtoError};

use std::collections::BTreeMap;
use time::{Tm, strptime};

/// Directory
#[derive(Clone, Debug, PartialEq)]
pub struct Directory {
    /// name
    pub name: String,
    /// last modified
    pub last_mod: Tm,
}

impl FromMap for Directory {
    fn from_map(map: BTreeMap<String, String>) -> Result<Directory, Error> {
        Ok(Directory {
               name: try!(map.get("directory").map(|v| v.to_owned()).ok_or(Error::Proto(ProtoError::NoField("directory")))),
               last_mod: try!(map.get("Last-Modified")
                .ok_or(Error::Proto(ProtoError::NoField("Last-Modified")))
                .and_then(|v| strptime(&*v, "%Y-%m-%dT%H:%M:%S%Z").map_err(From::from))),
           })
    }
}
