use ffi::AVRounding::*;
use ffi::*;

/** How to round? */
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Rounding {
    /** Round towards zero. */
    Zero,
    /** Round away from zero. */
    Infinity,
    /** Round toward -infinity. */
    Down,
    /** Round toward +infinity. */
    Up,
    /** Round to the nearest and halfway cases away from zero. */
    NearInfinity,
    /** Flag telling rescaling functions to pass `INT64_MIN/MAX` through unchanged,
     * avoiding special cases for `AV_NOPTS_VALUE` (aka `None` in rust). */
    PassMinMax,
}

impl From<AVRounding> for Rounding {
    #[inline(always)]
    fn from(value: AVRounding) -> Self {
        match value {
            AV_ROUND_ZERO => Rounding::Zero,
            AV_ROUND_INF => Rounding::Infinity,
            AV_ROUND_DOWN => Rounding::Down,
            AV_ROUND_UP => Rounding::Up,
            AV_ROUND_NEAR_INF => Rounding::NearInfinity,
            AV_ROUND_PASS_MINMAX => Rounding::PassMinMax,
        }
    }
}

impl From<Rounding> for AVRounding {
    #[inline(always)]
    fn from(value: Rounding) -> AVRounding {
        match value {
            Rounding::Zero => AV_ROUND_ZERO,
            Rounding::Infinity => AV_ROUND_INF,
            Rounding::Down => AV_ROUND_DOWN,
            Rounding::Up => AV_ROUND_UP,
            Rounding::NearInfinity => AV_ROUND_NEAR_INF,
            Rounding::PassMinMax => AV_ROUND_PASS_MINMAX,
        }
    }
}
