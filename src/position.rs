use anyhow::{anyhow, ensure, Context as _};

use crate::block::{Block, Blocks};
use crate::cost::{calc_hero_move_cost, calc_move_cost, Cost};
use crate::move_::{Move, MoveDst, MoveSrc, MOVE_SRC_ROW_11};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Position {
    hero_row: MoveSrc,
    blocks: Blocks,
    block_holding: Block,
    move_remain: u8,
}

impl Position {
    pub fn new(blocks: Blocks, block_holding: Block, move_remain: u8) -> Self {
        Self {
            hero_row: MOVE_SRC_ROW_11,
            blocks,
            block_holding,
            move_remain,
        }
    }

    pub fn hero_row(&self) -> MoveSrc {
        self.hero_row
    }

    pub fn blocks(&self) -> &Blocks {
        &self.blocks
    }

    pub fn block_holding(&self) -> Block {
        self.block_holding
    }

    pub fn move_remain(&self) -> u8 {
        self.move_remain
    }

    pub fn block_count(&self) -> usize {
        self.blocks.block_count()
    }

    /// 着手を行い、(結果, 総所要コスト, ブロック投げコスト) を返す。
    /// 着手が無効なら `None` を返す。
    ///
    /// ブロック投げコストは総所要コストから自機の移動コストを引いたもの。
    pub fn do_move(&self, mv: Move) -> Option<(Self, Cost, Cost)> {
        assert!(self.move_remain > 0);

        let cost_hero_move = calc_hero_move_cost(self.hero_row, mv.src());

        let hero_row = mv.src();
        let (blocks, block_holding, sq_last) = match mv.dst() {
            MoveDst::Horizontal(row) => self.blocks.do_move_hori(row, self.block_holding),
            MoveDst::Vertical(col) => self.blocks.do_move_vert(col, self.block_holding),
        }?;
        let move_remain = self.move_remain - 1;

        let cost_throw = calc_move_cost(mv.src(), sq_last);

        let pos_nxt = Self {
            hero_row,
            blocks,
            block_holding,
            move_remain,
        };

        let cost = cost_hero_move + cost_throw;

        Some((pos_nxt, cost, cost_throw))
    }
}

impl std::str::FromStr for Position {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (line, s) = s
            .split_once('\n')
            .ok_or_else(|| anyhow!("局面文字列の最初の行がない: '{s}'"))?;

        let tokens: Vec<_> = line.split_ascii_whitespace().collect();
        ensure!(
            tokens.len() == 3,
            "局面文字列の最初の行はちょうど 3 つのトークンを持たねばならない: '{line}'"
        );

        let hero_row: u8 = tokens[0]
            .parse()
            .with_context(|| format!("自機位置が数値でない: '{}'", tokens[0]))?;
        let hero_row =
            MoveSrc::from_inner(hero_row).ok_or_else(|| anyhow!("無効な自機位置: {hero_row}"))?;

        let block_holding: u8 = tokens[1]
            .parse()
            .with_context(|| format!("保持ブロックが数値でない: '{}'", tokens[1]))?;
        let block_holding = Block::from_inner(block_holding)
            .ok_or_else(|| anyhow!("無効な保持ブロック値: {block_holding}"))?;

        let move_remain: u8 = tokens[2]
            .parse()
            .with_context(|| format!("残り手数が数値でない: '{}'", tokens[2]))?;

        let blocks: Blocks = s.parse()?;

        Ok(Self {
            hero_row,
            blocks,
            block_holding,
            move_remain,
        })
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {} {}",
            self.hero_row.to_inner(),
            self.block_holding.to_inner(),
            self.move_remain
        )?;

        self.blocks.fmt(f)?;

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

    fn parse_position(s: impl AsRef<str>) -> Position {
        s.as_ref().parse().unwrap()
    }

    #[test]
    fn test_io() {
        let cases = [
            indoc! {"
                0 1 10
                ......
                ......
                ......
                ......
                ......
                ......
            "},
            indoc! {"
                11 5 10
                1.1.1.
                111111
                222222
                333333
                444444
                444444
            "},
        ];

        for case in cases {
            let pos = parse_position(case);
            assert_eq!(pos.to_string(), case);
        }
    }

    #[test]
    fn test_do_move() {
        let cases = [
            (
                indoc! {"
                    11 3 5
                    ......
                    ......
                    222222
                    333333
                    344444
                    311111
                "},
                Move::new(MOVE_SRC_ROW_9, MoveDst::Horizontal(BLOCKS_ROW_4)),
                indoc! {"
                    9 3 4
                    ......
                    ......
                    ......
                    .22222
                    .44444
                    211111
                "},
            ),
            (
                indoc! {"
                    0 1 5
                    ......
                    11....
                    11....
                    11....
                    11222.
                    312223
                "},
                Move::new(MOVE_SRC_ROW_5, MoveDst::Vertical(BLOCKS_COL_A)),
                indoc! {"
                    5 3 4
                    ......
                    .1....
                    .1....
                    .1....
                    .1222.
                    112223
                "},
            ),
        ];

        for (before, mv, after) in cases {
            let before = parse_position(before);
            let after = parse_position(after);
            let (after_actual, _, _) = before.do_move(mv).unwrap();
            assert_eq!(after_actual, after);
        }
    }
}
