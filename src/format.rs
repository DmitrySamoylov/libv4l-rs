use bitflags::bitflags;
use std::{fmt, str};

use crate::v4l_sys::*;
use crate::FourCC;

bitflags! {
    #[allow(clippy::unreadable_literal)]
    pub struct Flags : u32 {
        const COMPRESSED            = 0x0001;
        const EMULATED              = 0x0002;
        const CONTINUOUS_BITSTREAM  = 0x0004;
        const DYN_RESOLUTION        = 0x0008;
    }
}

impl From<u32> for Flags {
    fn from(flags: u32) -> Self {
        Flags::from_bits_truncate(flags)
    }
}

impl Into<u32> for Flags {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
/// Format description as returned by VIDIOC_ENUM_FMT
pub struct FormatDescription {
    pub index: u32,
    pub typ: u32,
    pub flags: Flags,
    pub description: String,
    pub fourcc: FourCC,
}

impl fmt::Display for FormatDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "index       : {}", self.index)?;
        writeln!(f, "type:       : {}", self.typ)?;
        writeln!(f, "flags:      : {}", self.flags)?;
        writeln!(f, "description : {}", self.description)?;
        writeln!(f, "fourcc      : {}", self.fourcc)?;
        Ok(())
    }
}

impl From<v4l2_fmtdesc> for FormatDescription {
    fn from(desc: v4l2_fmtdesc) -> Self {
        FormatDescription {
            index: desc.index,
            typ: desc.type_,
            flags: Flags::from(desc.flags),
            description: str::from_utf8(&desc.description).unwrap().to_string(),
            fourcc: FourCC::from(desc.pixelformat),
        }
    }
}
