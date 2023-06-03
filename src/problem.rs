use std::fmt::Write as _;

use anyhow::{anyhow, ensure, Context as _};

use crate::block::{Block, Blocks, BlocksCol, BlocksRow, BLOCKS_COL_A};
use crate::move_::{Move, MoveDst, MoveSrc};
use crate::position::Position;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProblemTile {
    Block(Block),
    Wall,
    Pipe,
}

impl ProblemTile {
    pub fn is_block(self) -> bool {
        matches!(self, Self::Block(_))
    }

    pub fn is_normal_block(self) -> bool {
        matches!(self, Self::Block(block) if block.is_normal())
    }

    pub fn is_wild_block(self) -> bool {
        matches!(self, Self::Block(Block::Wild))
    }

    pub fn is_wall(self) -> bool {
        matches!(self, Self::Wall)
    }

    pub fn is_pipe(self) -> bool {
        matches!(self, Self::Pipe)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProblemBoard([Option<ProblemTile>; Self::WIDTH * Self::HEIGHT]);

impl Default for ProblemBoard {
    fn default() -> Self {
        Self(std::array::from_fn(|_| None))
    }
}

impl ProblemBoard {
    const WIDTH: usize = 8;
    const HEIGHT: usize = 12;

    const CHAR_TILE_NONE: char = '.';
    const CHAR_TILE_BLOCK_NORMAL1: char = '1';
    const CHAR_TILE_BLOCK_NORMAL2: char = '2';
    const CHAR_TILE_BLOCK_NORMAL3: char = '3';
    const CHAR_TILE_BLOCK_NORMAL4: char = '4';
    const CHAR_TILE_BLOCK_WILD: char = '5';
    const CHAR_TILE_WALL: char = '#';
    const CHAR_TILE_PIPE: char = '|';

    pub fn new() -> Self {
        Self::default()
    }

    fn cr2idx(col: usize, row: usize) -> usize {
        Self::WIDTH * row + col
    }

    fn tile_to_char(tile: Option<ProblemTile>) -> char {
        match tile {
            None => Self::CHAR_TILE_NONE,
            Some(ProblemTile::Block(Block::Normal1)) => Self::CHAR_TILE_BLOCK_NORMAL1,
            Some(ProblemTile::Block(Block::Normal2)) => Self::CHAR_TILE_BLOCK_NORMAL2,
            Some(ProblemTile::Block(Block::Normal3)) => Self::CHAR_TILE_BLOCK_NORMAL3,
            Some(ProblemTile::Block(Block::Normal4)) => Self::CHAR_TILE_BLOCK_NORMAL4,
            Some(ProblemTile::Block(Block::Wild)) => Self::CHAR_TILE_BLOCK_WILD,
            Some(ProblemTile::Wall) => Self::CHAR_TILE_WALL,
            Some(ProblemTile::Pipe) => Self::CHAR_TILE_PIPE,
        }
    }

    fn char_to_tile(ch: char) -> anyhow::Result<Option<ProblemTile>> {
        match ch {
            Self::CHAR_TILE_NONE => Ok(None),
            Self::CHAR_TILE_BLOCK_NORMAL1 => Ok(Some(ProblemTile::Block(Block::Normal1))),
            Self::CHAR_TILE_BLOCK_NORMAL2 => Ok(Some(ProblemTile::Block(Block::Normal2))),
            Self::CHAR_TILE_BLOCK_NORMAL3 => Ok(Some(ProblemTile::Block(Block::Normal3))),
            Self::CHAR_TILE_BLOCK_NORMAL4 => Ok(Some(ProblemTile::Block(Block::Normal4))),
            Self::CHAR_TILE_BLOCK_WILD => Ok(Some(ProblemTile::Block(Block::Wild))),
            Self::CHAR_TILE_WALL => Ok(Some(ProblemTile::Wall)),
            Self::CHAR_TILE_PIPE => Ok(Some(ProblemTile::Pipe)),
            _ => Err(anyhow!("無効な盤面タイル文字: '{ch}'")),
        }
    }
}

impl std::ops::Index<(usize, usize)> for ProblemBoard {
    type Output = Option<ProblemTile>;

    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        &self.0[Self::cr2idx(col, row)]
    }
}

impl std::ops::IndexMut<(usize, usize)> for ProblemBoard {
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
        &mut self.0[Self::cr2idx(col, row)]
    }
}

impl std::str::FromStr for ProblemBoard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        ensure!(
            lines.len() == Self::HEIGHT,
            "盤面文字列はちょうど {} 行でなければならない",
            Self::HEIGHT
        );

        let mut this = Self::new();

        for (row, line) in lines.into_iter().enumerate() {
            let chars: Vec<_> = line.chars().collect();
            ensure!(
                chars.len() == Self::WIDTH,
                "盤面文字列の行はちょうど {} 文字でなければならない",
                Self::WIDTH
            );

            for (col, ch) in chars.into_iter().enumerate() {
                let tile = Self::char_to_tile(ch)?;
                this[(col, row)] = tile;
            }
        }

        Ok(this)
    }
}

impl std::fmt::Display for ProblemBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..Self::HEIGHT {
            for col in 0..Self::WIDTH {
                let tile = self[(col, row)];
                let ch = Self::tile_to_char(tile);
                f.write_char(ch)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Problem {
    board: ProblemBoard,
    block_holding: Block,
    move_remain: u8,
}

impl Problem {
    pub fn new(board: ProblemBoard, block_holding: Block, move_remain: u8) -> anyhow::Result<Self> {
        /// 与えられたマスが左下 6x6 の範囲内かどうかを返す。
        fn is_blocks_area(col: usize, row: usize) -> bool {
            (0..6).contains(&col) && (6..12).contains(&row)
        }

        // 問題が ADVANCE モードの制約を満たしているかどうかチェック。
        for (row, col) in itertools::iproduct!(0..ProblemBoard::HEIGHT, 0..ProblemBoard::WIDTH) {
            let tile = board[(col, row)];
            if is_blocks_area(col, row) {
                ensure!(
                    tile.map_or(true, |tile| tile.is_normal_block()),
                    "左下 6x6 は空白または通常ブロックでなければならない"
                );
            } else {
                ensure!(
                    !tile.is_some_and(ProblemTile::is_block),
                    "左下 6x6 の範囲外にブロックがあってはならない"
                );
                if tile.is_some_and(ProblemTile::is_wall) {
                    ensure!(
                        row == 0 || board[(col, row - 1)].is_some_and(ProblemTile::is_wall),
                        "壁の上には壁がなければならない"
                    );
                }
            }
        }

        Ok(Self {
            board,
            block_holding,
            move_remain,
        })
    }

    pub fn board(&self) -> &ProblemBoard {
        &self.board
    }

    pub fn block_holding(&self) -> Block {
        self.block_holding
    }

    pub fn move_remain(&self) -> u8 {
        self.move_remain
    }

    pub fn to_position_and_moves(&self) -> (Position, Vec<Move>) {
        let mut blocks = Blocks::new();

        for (prow, brow) in std::iter::zip(6..12, BlocksRow::all()) {
            for (pcol, bcol) in std::iter::zip(0..6, BlocksCol::all()) {
                let block = match self.board[(pcol, prow)] {
                    None => None,
                    Some(ProblemTile::Block(block)) => {
                        assert!(block.is_normal());
                        Some(block)
                    }
                    _ => unreachable!("左下 6x6 は空白または通常ブロックのはず"),
                };
                blocks[(bcol, brow)] = block;
            }
        }

        let pos = Position::new(blocks, self.block_holding, self.move_remain);

        // 各行からブロックを投げたときの着手を求め、有効なもののみを集める。
        let moves: Vec<_> = MoveSrc::all()
            .into_iter()
            .rev()
            .filter_map(|src| {
                // この行からブロックを横に投げたときに最初に当たるタイルとその列を求める。
                let col_tile = (0..ProblemBoard::WIDTH)
                    .rev()
                    .find_map(|col| self.board[(col, src.to_index())].map(|tile| (col, tile)));
                match col_tile {
                    // 当たるタイルがなければ一番奥まで行って落ちる着手となる。
                    None => {
                        let dst = MoveDst::Vertical(BLOCKS_COL_A);
                        Some(Move::new(src, dst))
                    }
                    // ブロックに当たるなら横に投げる着手となる。
                    Some((_, ProblemTile::Block(_))) => {
                        let brow = BlocksRow::try_from(src).unwrap();
                        let dst = MoveDst::Horizontal(brow);
                        Some(Move::new(src, dst))
                    }
                    // 壁またはパイプに当たるならそこから落ちる着手となる。
                    // ブロックに当たらないなら無効とする。
                    Some((col, _)) => {
                        let col = col + 1;
                        let block_exists = (src.to_index()..ProblemBoard::HEIGHT)
                            .any(|row| self.board[(col, row)].is_some_and(ProblemTile::is_block));
                        block_exists.then(|| {
                            let bcol = BlocksCol::from_inner((col + 1) as u8).unwrap();
                            let dst = MoveDst::Vertical(bcol);
                            Move::new(src, dst)
                        })
                    }
                }
            })
            .collect();

        (pos, moves)
    }
}

impl std::str::FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (line, s) = s
            .split_once('\n')
            .ok_or_else(|| anyhow!("問題文字列の最初の行がない: '{s}'"))?;

        let tokens: Vec<_> = line.split_ascii_whitespace().collect();
        ensure!(
            tokens.len() == 2,
            "問題文字列の最初の行はちょうど 2 つのトークンを持たねばならない: '{line}'"
        );

        let block_holding: u8 = tokens[0]
            .parse()
            .with_context(|| format!("保持ブロックが数値でない: '{}'", tokens[0]))?;
        let block_holding = Block::from_inner(block_holding)
            .ok_or_else(|| anyhow!("無効な保持ブロック値: {block_holding}"))?;

        let move_remain: u8 = tokens[1]
            .parse()
            .with_context(|| format!("残り手数が数値でない: '{}'", tokens[1]))?;

        let board: ProblemBoard = s.parse()?;

        let this = Self::new(board, block_holding, move_remain)
            .context("問題が ADVANCE モードの制約を満たしていない")?;

        Ok(this)
    }
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.block_holding.to_inner(), self.move_remain)?;

        self.board.fmt(f)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::block::*;
    use crate::move_::*;

    use super::*;

    fn parse_problem(s: impl AsRef<str>) -> Problem {
        s.as_ref().parse().unwrap()
    }

    fn parse_position(s: impl AsRef<str>) -> Position {
        s.as_ref().parse().unwrap()
    }

    #[test]
    fn test_io() {
        let s = indoc! {"
            2 33
            #####...
            ##......
            #.......
            ........
            ........
            ........
            311432..
            222242|.
            334422..
            422224|.
            344244..
            133344..
        "};

        let problem = parse_problem(s);
        assert_eq!(problem.to_string(), s);
    }

    #[test]
    fn test_to_position_and_moves() {
        let s_problem = indoc! {"
            2 33
            #####...
            ##......
            #.......
            ........
            ........
            ........
            31143...
            22224.|.
            33442...
            42222.|.
            34424...
            13334...
        "};

        let s_pos = indoc! {"
            11 2 33
            31143.
            22224.
            33442.
            42222.
            34424.
            13334.
        "};

        let problem = parse_problem(s_problem);
        let pos = parse_position(s_pos);
        let moves = vec![
            Move::new(MOVE_SRC_ROW_11, MoveDst::Horizontal(BLOCKS_ROW_6)),
            Move::new(MOVE_SRC_ROW_10, MoveDst::Horizontal(BLOCKS_ROW_5)),
            Move::new(MOVE_SRC_ROW_8, MoveDst::Horizontal(BLOCKS_ROW_3)),
            Move::new(MOVE_SRC_ROW_6, MoveDst::Horizontal(BLOCKS_ROW_1)),
            Move::new(MOVE_SRC_ROW_5, MoveDst::Vertical(BLOCKS_COL_A)),
            Move::new(MOVE_SRC_ROW_4, MoveDst::Vertical(BLOCKS_COL_A)),
            Move::new(MOVE_SRC_ROW_3, MoveDst::Vertical(BLOCKS_COL_A)),
            Move::new(MOVE_SRC_ROW_2, MoveDst::Vertical(BLOCKS_COL_B)),
            Move::new(MOVE_SRC_ROW_1, MoveDst::Vertical(BLOCKS_COL_C)),
        ];

        assert_eq!(problem.to_position_and_moves(), (pos, moves));
    }
}
