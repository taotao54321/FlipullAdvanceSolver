use anyhow::anyhow;

use crate::block::{BlocksCol, BlocksRow};

/// ブロックをどの行から投げるか。
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MoveSrc {
    Row0 = 0,
    Row1,
    Row2,
    Row3,
    Row4,
    Row5,
    Row6,
    Row7,
    Row8,
    Row9,
    Row10,
    Row11,
}

pub const MOVE_SRC_ROW_0: MoveSrc = MoveSrc::Row0;
pub const MOVE_SRC_ROW_1: MoveSrc = MoveSrc::Row1;
pub const MOVE_SRC_ROW_2: MoveSrc = MoveSrc::Row2;
pub const MOVE_SRC_ROW_3: MoveSrc = MoveSrc::Row3;
pub const MOVE_SRC_ROW_4: MoveSrc = MoveSrc::Row4;
pub const MOVE_SRC_ROW_5: MoveSrc = MoveSrc::Row5;
pub const MOVE_SRC_ROW_6: MoveSrc = MoveSrc::Row6;
pub const MOVE_SRC_ROW_7: MoveSrc = MoveSrc::Row7;
pub const MOVE_SRC_ROW_8: MoveSrc = MoveSrc::Row8;
pub const MOVE_SRC_ROW_9: MoveSrc = MoveSrc::Row9;
pub const MOVE_SRC_ROW_10: MoveSrc = MoveSrc::Row10;
pub const MOVE_SRC_ROW_11: MoveSrc = MoveSrc::Row11;

impl MoveSrc {
    pub const NUM: usize = 12;

    pub const MIN_VALUE: u8 = 0;
    pub const MAX_VALUE: u8 = 11;

    pub fn from_inner(inner: u8) -> Option<Self> {
        Self::is_valid(inner).then(|| unsafe { Self::from_inner_unchecked(inner) })
    }

    /// # Safety
    ///
    /// `inner` は有効値でなければならない。
    pub unsafe fn from_inner_unchecked(inner: u8) -> Self {
        assert!(Self::is_valid(inner));

        std::mem::transmute(inner)
    }

    pub const fn to_inner(self) -> u8 {
        self as u8
    }

    pub const fn to_index(self) -> usize {
        self.to_inner() as usize
    }

    pub fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }

    pub fn all() -> [Self; Self::NUM] {
        [
            Self::Row0,
            Self::Row1,
            Self::Row2,
            Self::Row3,
            Self::Row4,
            Self::Row5,
            Self::Row6,
            Self::Row7,
            Self::Row8,
            Self::Row9,
            Self::Row10,
            Self::Row11,
        ]
    }
}

impl TryFrom<MoveSrc> for BlocksRow {
    type Error = anyhow::Error;

    fn try_from(src: MoveSrc) -> Result<Self, Self::Error> {
        match src {
            MoveSrc::Row6 => Ok(Self::Row1),
            MoveSrc::Row7 => Ok(Self::Row2),
            MoveSrc::Row8 => Ok(Self::Row3),
            MoveSrc::Row9 => Ok(Self::Row4),
            MoveSrc::Row10 => Ok(Self::Row5),
            MoveSrc::Row11 => Ok(Self::Row6),
            _ => Err(anyhow!("ブロック領域の行ではない: {src:?}")),
        }
    }
}

/// ブロックをどの行/列に投げるか。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoveDst {
    Horizontal(BlocksRow),
    Vertical(BlocksCol),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Move {
    src: MoveSrc,
    dst: MoveDst,
}

impl Move {
    pub fn new(src: MoveSrc, dst: MoveDst) -> Self {
        Self { src, dst }
    }

    pub fn src(self) -> MoveSrc {
        self.src
    }

    pub fn dst(self) -> MoveDst {
        self.dst
    }
}
