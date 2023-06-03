use std::fmt::Write as _;

use anyhow::{anyhow, ensure};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Block {
    Normal1 = 1,
    Normal2,
    Normal3,
    Normal4,
    Wild,
}

impl Block {
    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 5;

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

    pub fn to_inner(self) -> u8 {
        self as u8
    }

    pub fn is_normal(self) -> bool {
        !self.is_wild()
    }

    pub fn is_wild(self) -> bool {
        self == Self::Wild
    }

    pub fn can_erase(self, other: Self) -> bool {
        match self {
            Self::Wild => true,
            _ => self == other,
        }
    }

    pub fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }
}

/// 盤面左下 6x6 のブロック領域の列。
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlocksCol {
    ColA = 1,
    ColB,
    ColC,
    ColD,
    ColE,
    ColF,
}

pub const BLOCKS_COL_A: BlocksCol = BlocksCol::ColA;
pub const BLOCKS_COL_B: BlocksCol = BlocksCol::ColB;
pub const BLOCKS_COL_C: BlocksCol = BlocksCol::ColC;
pub const BLOCKS_COL_D: BlocksCol = BlocksCol::ColD;
pub const BLOCKS_COL_E: BlocksCol = BlocksCol::ColE;
pub const BLOCKS_COL_F: BlocksCol = BlocksCol::ColF;

impl BlocksCol {
    pub const NUM: usize = 6;

    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 6;

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
        (self.to_inner() - 1) as usize
    }

    pub fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }

    pub fn all() -> [Self; Self::NUM] {
        [
            BLOCKS_COL_A,
            BLOCKS_COL_B,
            BLOCKS_COL_C,
            BLOCKS_COL_D,
            BLOCKS_COL_E,
            BLOCKS_COL_F,
        ]
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlocksRow {
    Row1 = 1,
    Row2,
    Row3,
    Row4,
    Row5,
    Row6,
}

pub const BLOCKS_ROW_1: BlocksRow = BlocksRow::Row1;
pub const BLOCKS_ROW_2: BlocksRow = BlocksRow::Row2;
pub const BLOCKS_ROW_3: BlocksRow = BlocksRow::Row3;
pub const BLOCKS_ROW_4: BlocksRow = BlocksRow::Row4;
pub const BLOCKS_ROW_5: BlocksRow = BlocksRow::Row5;
pub const BLOCKS_ROW_6: BlocksRow = BlocksRow::Row6;

impl BlocksRow {
    pub const NUM: usize = 6;

    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 6;

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
        (self.to_inner() - 1) as usize
    }

    pub fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }

    pub fn all() -> [Self; Self::NUM] {
        [
            BLOCKS_ROW_1,
            BLOCKS_ROW_2,
            BLOCKS_ROW_3,
            BLOCKS_ROW_4,
            BLOCKS_ROW_5,
            BLOCKS_ROW_6,
        ]
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlocksSquare {
    SqA1 = 1,
    SqB1,
    SqC1,
    SqD1,
    SqE1,
    SqF1,
    SqA2,
    SqB2,
    SqC2,
    SqD2,
    SqE2,
    SqF2,
    SqA3,
    SqB3,
    SqC3,
    SqD3,
    SqE3,
    SqF3,
    SqA4,
    SqB4,
    SqC4,
    SqD4,
    SqE4,
    SqF4,
    SqA5,
    SqB5,
    SqC5,
    SqD5,
    SqE5,
    SqF5,
    SqA6,
    SqB6,
    SqC6,
    SqD6,
    SqE6,
    SqF6,
}

impl BlocksSquare {
    pub const NUM: usize = 36;

    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 36;

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

    pub fn new(col: BlocksCol, row: BlocksRow) -> Self {
        let inner = 6 * (row.to_inner() - 1) + col.to_inner();

        Self::from_inner(inner).unwrap()
    }

    pub const fn to_inner(self) -> u8 {
        self as u8
    }

    pub const fn to_index(self) -> usize {
        (self.to_inner() - 1) as usize
    }

    pub fn col(self) -> BlocksCol {
        let col = (self.to_inner() - 1) % 6 + 1;

        BlocksCol::from_inner(col).unwrap()
    }

    pub fn row(self) -> BlocksRow {
        let row = (self.to_inner() - 1) / 6 + 1;

        BlocksRow::from_inner(row).unwrap()
    }

    pub fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }
}

/// 盤面左下 6x6 のブロック領域。
///
/// 左と下に番兵を設けている:
///
/// ```text
///    ABCDEF
/// 1 #......
/// 2 #......
/// 3 #......
/// 4 #......
/// 5 #......
/// 6 #......
///   #######
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Blocks([Option<Block>; 7 * 7]);

impl Default for Blocks {
    fn default() -> Self {
        Self(std::array::from_fn(|_| None))
    }
}

impl Blocks {
    const DIR_U: isize = -7;
    const DIR_L: isize = -1;
    const DIR_D: isize = 7;

    const CHAR_NONE: char = '.';
    const CHAR_BLOCK_1: char = '1';
    const CHAR_BLOCK_2: char = '2';
    const CHAR_BLOCK_3: char = '3';
    const CHAR_BLOCK_4: char = '4';

    pub fn new() -> Self {
        Self::default()
    }

    pub fn block_count(&self) -> usize {
        self.0.iter().copied().filter(Option::is_some).count()
    }

    /// ブロックを横方向に投げ込み、(結果, 次の保持ブロック, 置換前に最後にブロックが通った位置) を返す。
    /// 着手が無効(盤面が変化しない)なら `None` を返す。
    pub fn do_move_hori(
        &self,
        row: BlocksRow,
        block_move: Block,
    ) -> Option<(Self, Block, BlocksSquare)> {
        let start = Self::cr2idx(BLOCKS_COL_F, row);

        self.do_move_impl(start, Self::DIR_L, block_move)
    }

    /// ブロックを縦方向に投げ込み、(結果, 次の保持ブロック, 置換前に最後にブロックが通った位置) を返す。
    /// 着手が無効(盤面が変化しない)なら `None` を返す。
    pub fn do_move_vert(
        &self,
        col: BlocksCol,
        block_move: Block,
    ) -> Option<(Self, Block, BlocksSquare)> {
        let start = Self::cr2idx(col, BLOCKS_ROW_1);

        self.do_move_impl(start, Self::DIR_D, block_move)
    }

    fn do_move_impl(
        &self,
        start: usize,
        dir: isize,
        block_move: Block,
    ) -> Option<(Self, Block, BlocksSquare)> {
        let vert = dir == Self::DIR_D;

        let mut idxs = Self::idx_iter(start, dir);

        // 最初に当たるブロックとその位置を得る。当たらないなら着手は無効。
        let (idx_first, block_first) = idxs
            .by_ref()
            .find_map(|idx| self.0[idx].map(|block| (idx, block)))?;

        // 最初に当たったブロックが消せないなら着手は無効。
        if !block_move.can_erase(block_first) {
            return None;
        }

        let mut blocks_res = self.clone();
        let mut block_holding_nxt = block_first;
        let mut idx_last = idx_first;

        macro_rules! erase {
            ($idx:expr) => {{
                if vert {
                    blocks_res.0[$idx] = None;
                } else {
                    blocks_res.erase_shift($idx);
                }
            }};
        }

        // 最初に当たったブロックを消す。
        erase!(idx_first);

        // その後の移動の処理。
        for idx in idxs {
            if let Some(block) = self.0[idx] {
                if block_first == block {
                    // 当たったブロックが block_first と同種なら単に消す。
                    erase!(idx);
                } else {
                    // block_first と違う種類のブロックに当たったら置換を行い、そこで止まる。
                    // ADVANCE モードでは盤面にワイルドカードは現れないことに注意。
                    blocks_res.0[idx] = Some(block_first);
                    block_holding_nxt = block;
                    break;
                }
            } else {
                // ブロックに当たらないなら素通り。
            }
            idx_last = idx;
        }

        let sq_last = Self::idx2sq(idx_last);

        Some((blocks_res, block_holding_nxt, sq_last))
    }

    fn erase_shift(&mut self, mut idx: usize) {
        while let Some(idx_nxt) = idx.checked_add_signed(Self::DIR_U) {
            self.0[idx] = self.0[idx_nxt];
            idx = idx_nxt;
        }
        self.0[idx] = None;
    }

    fn idx_iter(start: usize, dir: isize) -> impl Iterator<Item = usize> {
        #[rustfmt::skip]
        const SENTINELS: [bool; 7 * 7] = [
            true, false, false, false, false, false, false,
            true, false, false, false, false, false, false,
            true, false, false, false, false, false, false,
            true, false, false, false, false, false, false,
            true, false, false, false, false, false, false,
            true, false, false, false, false, false, false,
            true, true,  true,  true,  true,  true,  true,
        ];

        std::iter::successors(Some((start, dir)), |&(idx, dir)| {
            let idx_nxt = idx.checked_add_signed(dir).unwrap();

            if SENTINELS[idx_nxt] {
                if dir == Self::DIR_L {
                    let idx_nxt = idx.checked_add_signed(Self::DIR_D).unwrap();
                    (!SENTINELS[idx_nxt]).then_some((idx_nxt, Self::DIR_D))
                } else {
                    None
                }
            } else {
                Some((idx_nxt, dir))
            }
        })
        .map(|(idx, _)| idx)
    }

    fn cr2idx(col: BlocksCol, row: BlocksRow) -> usize {
        7 * row.to_index() + col.to_index() + 1
    }

    fn sq2idx(sq: BlocksSquare) -> usize {
        Self::cr2idx(sq.col(), sq.row())
    }

    fn idx2sq(idx: usize) -> BlocksSquare {
        let col = (idx % 7) as u8;
        let col = BlocksCol::from_inner(col).unwrap();

        let row = (idx / 7 + 1) as u8;
        let row = BlocksRow::from_inner(row).unwrap();

        BlocksSquare::new(col, row)
    }

    fn block_to_char(block: Option<Block>) -> char {
        match block {
            None => Self::CHAR_NONE,
            Some(Block::Normal1) => Self::CHAR_BLOCK_1,
            Some(Block::Normal2) => Self::CHAR_BLOCK_2,
            Some(Block::Normal3) => Self::CHAR_BLOCK_3,
            Some(Block::Normal4) => Self::CHAR_BLOCK_4,
            Some(Block::Wild) => unreachable!("Blocks 内にワイルドカードがある"),
        }
    }

    fn char_to_block(ch: char) -> anyhow::Result<Option<Block>> {
        match ch {
            Self::CHAR_NONE => Ok(None),
            Self::CHAR_BLOCK_1 => Ok(Some(Block::Normal1)),
            Self::CHAR_BLOCK_2 => Ok(Some(Block::Normal2)),
            Self::CHAR_BLOCK_3 => Ok(Some(Block::Normal3)),
            Self::CHAR_BLOCK_4 => Ok(Some(Block::Normal4)),
            _ => Err(anyhow!("無効な Blocks 内ブロック文字: '{ch}'")),
        }
    }
}

impl std::ops::Index<(BlocksCol, BlocksRow)> for Blocks {
    type Output = Option<Block>;

    fn index(&self, (col, row): (BlocksCol, BlocksRow)) -> &Self::Output {
        &self.0[Self::cr2idx(col, row)]
    }
}

impl std::ops::IndexMut<(BlocksCol, BlocksRow)> for Blocks {
    fn index_mut(&mut self, (col, row): (BlocksCol, BlocksRow)) -> &mut Self::Output {
        &mut self.0[Self::cr2idx(col, row)]
    }
}

impl std::ops::Index<BlocksSquare> for Blocks {
    type Output = Option<Block>;

    fn index(&self, sq: BlocksSquare) -> &Self::Output {
        &self.0[Self::sq2idx(sq)]
    }
}

impl std::ops::IndexMut<BlocksSquare> for Blocks {
    fn index_mut(&mut self, sq: BlocksSquare) -> &mut Self::Output {
        &mut self.0[Self::sq2idx(sq)]
    }
}

impl std::str::FromStr for Blocks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        ensure!(
            lines.len() == 6,
            "Blocks 文字列はちょうど 6 行でなければならない"
        );

        let mut this = Self::new();

        for (row, line) in std::iter::zip(BlocksRow::all(), lines) {
            let chars: Vec<_> = line.chars().collect();
            ensure!(
                chars.len() == 6,
                "Blocks 文字列の行はちょうど 6 文字でなければならない"
            );

            for (col, ch) in std::iter::zip(BlocksCol::all(), chars) {
                let block = Self::char_to_block(ch)?;
                this[(col, row)] = block;
            }
        }

        Ok(this)
    }
}

impl std::fmt::Display for Blocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in BlocksRow::all() {
            for col in BlocksCol::all() {
                let block = self[(col, row)];
                f.write_char(Self::block_to_char(block))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    fn parse_blocks(s: impl AsRef<str>) -> Blocks {
        s.as_ref().parse().unwrap()
    }

    #[test]
    fn test_blocks_io() {
        let cases = [
            indoc! {"
                ......
                ......
                ......
                ......
                ......
                ......
            "},
            indoc! {"
                1.1.1.
                111111
                222222
                333333
                444444
                444444
            "},
        ];

        for case in cases {
            let blocks = parse_blocks(case);
            assert_eq!(blocks.to_string(), case);
        }
    }

    #[test]
    fn test_blocks_do_move_hori() {
        assert_eq!(
            Blocks::new().do_move_hori(BLOCKS_ROW_1, Block::Normal1),
            None
        );

        let cases = [
            (
                indoc! {"
                    2.1111
                    333311
                    222222
                    333333
                    444444
                    333333
                "},
                BLOCKS_ROW_1,
                Block::Normal1,
                indoc! {"
                    1.....
                    333311
                    222222
                    333333
                    444444
                    333333
                "},
                Block::Normal2,
                BlocksSquare::SqB1,
            ),
            (
                indoc! {"
                    211111
                    333311
                    222222
                    333333
                    444444
                    333333
                "},
                BLOCKS_ROW_6,
                Block::Wild,
                indoc! {"
                    ......
                    211111
                    333311
                    222222
                    333333
                    444444
                "},
                Block::Normal3,
                BlocksSquare::SqA6,
            ),
            (
                indoc! {"
                    ......
                    ......
                    222222
                    333333
                    344444
                    411111
                "},
                BLOCKS_ROW_4,
                Block::Normal3,
                indoc! {"
                    ......
                    ......
                    ......
                    .22222
                    244444
                    311111
                "},
                Block::Normal4,
                BlocksSquare::SqA5,
            ),
        ];

        for (before, row, block, after, block_res, sq_res) in cases {
            let before = parse_blocks(before);
            let after = parse_blocks(after);
            assert_eq!(
                before.do_move_hori(row, block).unwrap(),
                (after, block_res, sq_res)
            );
        }
    }

    #[test]
    fn test_blocks_do_move_vert() {
        assert_eq!(
            Blocks::new().do_move_vert(BLOCKS_COL_A, Block::Normal1),
            None
        );

        let cases = [
            (
                indoc! {"
                    .....1
                    ....21
                    ...322
                    ..4322
                    ..4322
                    ..4322
                "},
                BLOCKS_COL_F,
                Block::Normal1,
                indoc! {"
                    ......
                    ....2.
                    ...321
                    ..4322
                    ..4322
                    ..4322
                "},
                Block::Normal2,
                BlocksSquare::SqF2,
            ),
            (
                indoc! {"
                    ......
                    11....
                    11....
                    11....
                    11222.
                    112223
                "},
                BLOCKS_COL_A,
                Block::Wild,
                indoc! {"
                    ......
                    .1....
                    .1....
                    .1....
                    .1222.
                    .12223
                "},
                Block::Normal1,
                BlocksSquare::SqA6,
            ),
        ];

        for (before, col, block, after, block_res, sq_res) in cases {
            let before = parse_blocks(before);
            let after = parse_blocks(after);
            assert_eq!(
                before.do_move_vert(col, block).unwrap(),
                (after, block_res, sq_res)
            );
        }
    }
}
