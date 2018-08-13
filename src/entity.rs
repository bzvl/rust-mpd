//! This module defines entity data structures

use convert::{FromIter, FromMap};
use directory::Directory;
use playlist::Playlist;
use song::Song;
use error::{Error, ProtoError};

use std::collections::BTreeMap;
use std::mem;

/// Entity
#[derive(Clone, Debug, PartialEq)]
pub enum Entity {
    /// directory
    Directory(Directory),
    /// playlist
    Playlist(Playlist),
    /// file
    File(Song),
}

// XXX: tosongpath -> toentitypath and other addressing modes..
// ToPlaylistName
// // check with other apis

impl FromMap for Entity {
    fn from_map(map: BTreeMap<String, String>) -> Result<Self, Error> {
        if map.contains_key("directory") {
            Ok(Entity::Directory(FromMap::from_map(map)?))
        } else if map.contains_key("playlist") {
            Ok(Entity::Playlist(FromMap::from_map(map)?))
        } else if map.contains_key("file") {
            Ok(Entity::File(FromMap::from_map(map)?))
        } else {
            Err(Error::Proto(ProtoError::BadEntity))
        }
    }
}

// XXX: make this suck less
impl FromIter for Vec<Entity> {
    fn from_iter<I: Iterator<Item = Result<(String, String), Error>>>(iter: I) -> Result<Self, Error> {
        let mut result = Vec::new();
        let mut current = BTreeMap::new();
        for reply in iter {
            let (a, b) = reply?;
            let new = match &*a {
                "directory" => true,
                "playlist" => true,
                "file" => true,
                _ => false,
            };

            // XXX: such crap!
            if new && !current.is_empty() {
                result.push(Entity::from_map(mem::replace(&mut current, BTreeMap::new()))?);
            }

            current.insert(a, b);
        }

        if !current.is_empty() {
            result.push(Entity::from_map(mem::replace(&mut current, BTreeMap::new()))?);
        }

        Ok(result)
    }
}
