//! Store trait for cache interchangeability.

mod memory_store;
mod file_store;

pub use memory_store::*;
pub use file_store::*;

use std::fs::File;

use crate::arc::Archive;

pub const SECTOR_HEADER_SIZE: usize = 8;
pub const SECTOR_EXPANDED_HEADER_SIZE: usize = 10;
pub const SECTOR_DATA_SIZE: usize = 512;
pub const SECTOR_EXPANDED_DATA_SIZE: usize = 510;
pub const SECTOR_SIZE: usize = SECTOR_HEADER_SIZE + SECTOR_DATA_SIZE;

/// The internal storage for a cache with a way to read internal data.
pub trait Store: Sized {
    fn new(main_file: File) -> crate::Result<Self>;
    fn read(&self, archive: &Archive) -> crate::Result<Vec<u8>>;
}