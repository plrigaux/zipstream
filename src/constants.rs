use std::mem::size_of;

pub const FILE_HEADER_BASE_SIZE: u64 = (7 * size_of::<u16>() + 4 * size_of::<u32>()) as u64;
pub const DESCRIPTOR_SIZE: u64 = (4 * size_of::<u32>()) as u64;
pub const CENTRAL_DIRECTORY_ENTRY_BASE_SIZE: u64 =
    (11 * size_of::<u16>() + 6 * size_of::<u32>()) as u64;
pub const END_OF_CENTRAL_DIRECTORY_SIZE: u64 = (5 * size_of::<u16>() + 3 * size_of::<u32>()) as u64;
pub const FILE_HEADER_CRC_OFFSET: u64 = 14;

pub const CENTRAL_DIRECTORY_END_SIGNATURE: u32 = 0x06054b50;
pub const CENTRAL_DIRECTORY_ENTRY_SIGNATURE: u32 = 0x02014b50;
pub const LOCAL_FILE_HEADER_SIGNATURE: u32 = 0x04034b50; // Local file header signature.
pub const DATA_DESCRIPTOR_SIGNATURE: u32 = 0x08074b50; // Data descriptor signature.

pub const DEFAULT_VERSION: u8 = 46;
pub const UNIX: u8 = 3;
pub const VERSION_MADE_BY: u16 = (UNIX as u16) << 8 | DEFAULT_VERSION as u16;
