use std::convert::TryFrom;
use std::fmt;

use crate::FourCC;
use crate::{v4l_sys, v4l_sys::*, Fraction};

#[derive(Debug)]
/// Format description as returned by VIDIOC_ENUM_FRAMEINTERVALS
pub struct FrameInterval {
    pub index: u32,
    pub fourcc: FourCC,
    pub width: u32,
    pub height: u32,
    pub typ: u32,
    pub interval: FrameIntervalEnum,
}

impl fmt::Display for FrameInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "index    : {}", self.index)?;
        writeln!(f, "fourcc   : {}", self.fourcc)?;
        writeln!(f, "width    : {}", self.width)?;
        writeln!(f, "height   : {}", self.height)?;
        writeln!(f, "type     : {}", self.typ)?;
        writeln!(f, "interval : {}", self.interval)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum FrameIntervalEnum {
    Discrete(Fraction),
    Stepwise(Stepwise),
}

impl fmt::Display for FrameIntervalEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrameIntervalEnum::Discrete(val) => write!(f, "Discrete({})", val)?,
            FrameIntervalEnum::Stepwise(val) => write!(f, "Stepwise({})", val)?,
        }

        Ok(())
    }
}

impl TryFrom<v4l2_frmivalenum> for FrameIntervalEnum {
    type Error = String;

    fn try_from(desc: v4l2_frmivalenum) -> Result<Self, Self::Error> {
        unsafe {
            // Unsafe because of access to union __bindgen_anon_1
            match desc.type_ {
                v4l_sys::v4l2_frmivaltypes_V4L2_FRMIVAL_TYPE_DISCRETE => Ok(
                    FrameIntervalEnum::Discrete(Fraction::from(desc.__bindgen_anon_1.discrete)),
                ),
                v4l_sys::v4l2_frmivaltypes_V4L2_FRMIVAL_TYPE_CONTINUOUS
                | v4l_sys::v4l2_frmivaltypes_V4L2_FRMIVAL_TYPE_STEPWISE => Ok({
                    FrameIntervalEnum::Stepwise(Stepwise {
                        min: Fraction::from(desc.__bindgen_anon_1.stepwise.min),
                        max: Fraction::from(desc.__bindgen_anon_1.stepwise.max),
                        step: Fraction::from(desc.__bindgen_anon_1.stepwise.step),
                    })
                }),
                typ => Err(format!("Unknown frame size type: {}", typ)),
            }
        }
    }
}

#[derive(Debug)]
pub struct Stepwise {
    /// Minimum frame interval [s].
    pub min: Fraction,
    /// Maximum frame interval [s].
    pub max: Fraction,
    /// Frame interval step size [s].
    pub step: Fraction,
}

impl fmt::Display for Stepwise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} with step {}", self.min, self.max, self.step)?;
        Ok(())
    }
}

impl TryFrom<v4l2_frmivalenum> for FrameInterval {
    type Error = String;

    fn try_from(desc: v4l2_frmivalenum) -> Result<Self, Self::Error> {
        Ok(FrameInterval {
            index: desc.index,
            fourcc: FourCC::from(desc.pixel_format),
            width: desc.width,
            height: desc.height,
            typ: desc.type_,
            interval: FrameIntervalEnum::try_from(desc)?,
        })
    }
}
