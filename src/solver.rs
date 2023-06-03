use anyhow::{anyhow, ensure, Context as _};
use itertools::Itertools as _;
use log::info;

use crate::cost::{Cost, COST_CLEAR_ERASE_BLOCK};
use crate::move_::{Move, MoveSrc};
use crate::position::Position;
use crate::problem::Problem;

#[derive(Debug)]
pub struct Solution(Vec<MoveSrc>);

impl Solution {
    pub fn moves(&self) -> &[MoveSrc] {
        &self.0
    }

    pub fn verify(&self, problem: &Problem) -> anyhow::Result<Cost> {
        let (mut pos, moves) = problem.to_position_and_moves();
        let mut cost_total = 0;

        for (i, &src) in self.0.iter().enumerate() {
            ensure!(pos.move_remain() > 0, "{i} 番目の着手前に残り手数が尽きた");
            let mv = moves
                .iter()
                .copied()
                .find(|mv| mv.src() == src)
                .ok_or_else(|| anyhow!("{i} 番目の着手が不正: {src:?}"))?;
            let (pos_nxt, cost_mv) = pos
                .do_move(mv)
                .ok_or_else(|| anyhow!("{i} 番目の着手が不正: {mv:?}"))?;
            pos = pos_nxt;
            cost_total += cost_mv;
        }

        let stuck = moves.iter().all(|&mv| pos.do_move(mv).is_none());
        ensure!(stuck, "最後の局面でまだ合法手がある:\n{pos}");

        ensure!(pos.block_count() <= 3, "最後の局面が解けていない:\n{pos}");

        cost_total += COST_CLEAR_ERASE_BLOCK * pos.block_count() as Cost;

        Ok(cost_total)
    }
}

impl std::str::FromStr for Solution {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut srcs = Vec::<MoveSrc>::new();

        for (i, token) in s.split_ascii_whitespace().enumerate() {
            let src: u8 = token
                .parse()
                .with_context(|| format!("{i} 番目の着手が数値でない: '{token}'"))?;
            let src =
                MoveSrc::from_inner(src).ok_or_else(|| anyhow!("{i} 番目の着手が無効: '{src}'"))?;
            srcs.push(src);
        }

        Ok(Solution(srcs))
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().copied().map(MoveSrc::to_inner).join(" ")
        )
    }
}

pub fn solve_problem(problem: &Problem) -> Option<(Solution, Cost)> {
    let (pos, moves) = problem.to_position_and_moves();

    let mut solver = Solver::new();

    info!("search start");
    solver.solve(&moves, pos, 0);
    info!("search end");

    solver.best_solution.map(|solution| {
        let srcs = solution.into_iter().map(Move::src).collect();
        let solution = Solution(srcs);
        (solution, solver.best_cost)
    })
}

#[derive(Debug)]
struct Solver {
    best_solution: Option<Vec<Move>>,
    best_cost: Cost,
    cur_solution: Vec<Move>,
}

impl Solver {
    fn new() -> Self {
        Self {
            best_solution: None,
            best_cost: Cost::MAX,
            cur_solution: vec![],
        }
    }

    fn solve(&mut self, moves: &[Move], pos: Position, cost: Cost) {
        // 現局面が解けていると仮定したときの面クリアコストを求める。
        let cost_clear = COST_CLEAR_ERASE_BLOCK * pos.block_count() as Cost;

        // 現局面が解けていると仮定して面クリアコストを加算した結果が best_cost 以上ならば枝刈り。
        if cost + cost_clear >= self.best_cost {
            return;
        }

        let mut has_move = false;
        for &mv in moves {
            let Some((pos_nxt, cost_mv)) = pos.do_move(mv) else {
                continue;
            };
            has_move = true;
            self.cur_solution.push(mv);
            self.solve(moves, pos_nxt, cost + cost_mv);
            self.cur_solution.pop().unwrap();
        }

        // 現局面が実際に解けていれば最適解を更新(更新されないケースは事前に枝刈りしていることに注意)。
        if !has_move && pos.block_count() <= 3 {
            self.best_solution = Some(self.cur_solution.clone());
            self.best_cost = cost + cost_clear;
            info!("improve: {} {:?}", self.best_cost, self.best_solution);
        }
    }
}
