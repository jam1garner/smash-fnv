//! # fnv_lib
//!
//! fnv_lib is a library for reading and writing `sound_volume_fighter_num_table.fnv` files from Super Smash Bros. for Nintendo 3DS and Wii U and Super Smash Bros. Ultimate. Not to be confused with [Fowler–Noll–Vo](https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function).
use std::{
    fs,
    io::{Cursor, Read, Seek, Write},
    path::Path,
};

use binrw::{binrw, BinReaderExt, BinResult, BinWrite};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The container type for groups of audio gain parameters according to the number of fighters in-game.
#[binrw]
#[brw(magic = b"FNV\0", little)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug)]
pub struct FnvFile {
    #[br(temp)]
    #[bw(calc = 1u32)]
    unk1: u32,

    #[br(temp)]
    #[bw(calc = entries.len() as u32)]
    entry_count: u32,

    #[br(count = entry_count)]
    pub entries: Vec<FnvEntry>,
}

impl FnvFile {
    /// Reads the data from the given reader.
    pub fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self> {
        let fnv = reader.read_le::<Self>()?;

        Ok(fnv)
    }

    /// Reads the data from the given file path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        let mut file = Cursor::new(fs::read(path)?);
        let fnv = file.read_le::<Self>()?;

        Ok(fnv)
    }

    /// Writes the data to the given writer.
    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> BinResult<()> {
        self.write_le(writer)
    }

    /// Writes the data to the given file path.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> BinResult<()> {
        let mut cursor = Cursor::new(Vec::new());

        self.write_le(&mut cursor)?;
        fs::write(path, cursor.get_mut())?;

        Ok(())
    }
}

/// A group of audio gain parameters according to the number of fighters in-game.
#[binrw]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug)]
pub struct FnvEntry {
    pub fighter_num: u32,
    pub volume: Volume,
}

/// A group of audio gain parameters for different classes of sounds.
#[binrw]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug)]
pub struct Volume {
    pub other: f32,
    pub sound_attr: f32,
    pub se_fighter_step: f32,
    pub se_fighter_landing: f32,
    pub se_collision_step: f32,
    pub se_collision_landing: f32,
    pub se_stage: f32,
    pub bgm: f32,
}
