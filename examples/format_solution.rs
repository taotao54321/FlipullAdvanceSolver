use std::path::PathBuf;

use anyhow::Context as _;
use clap::{Parser, ValueEnum};

use flipull_advance_solver::*;

#[derive(Debug, Parser)]
struct Cli {
    /// フォーマット。
    #[arg(long, value_enum, default_value_t = Format::Pretty)]
    format: Format,

    /// 問題ファイル。
    path_problem: PathBuf,

    /// 解ファイル。
    path_solution: PathBuf,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum Format {
    /// 着手ごとに途中経過を出力する。
    Pretty,

    /// FCEUX の TAS Editor にペーストできるムービーを出力する。
    Fceux,

    /// Neshawk の TAStudio にペーストできるムービーを出力する。
    Neshawk,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let problem = std::fs::read_to_string(&cli.path_problem).with_context(|| {
        format!(
            "問題ファイル '{}' を読み取れない",
            cli.path_problem.display()
        )
    })?;
    let problem: Problem = problem.parse()?;

    let solution = std::fs::read_to_string(&cli.path_solution).with_context(|| {
        format!(
            "解ファイル '{}' を読み取れない",
            cli.path_solution.display()
        )
    })?;
    let solution: Solution = solution.parse()?;

    let (pos, moves) = problem.to_position_and_moves();

    match cli.format {
        Format::Pretty => format_pretty(pos, &moves, &solution),
        Format::Fceux => format_fceux(pos, &moves, &solution),
        Format::Neshawk => format_neshawk(pos, &moves, &solution),
    }

    Ok(())
}

fn format_pretty(mut pos: Position, moves: &[Move], solution: &Solution) {
    println!("{pos}");

    let mut cost_total = 0;

    for (i, &src) in solution.moves().iter().enumerate() {
        assert_ne!(pos.move_remain(), 0);

        let mv = moves.iter().copied().find(|mv| mv.src() == src).unwrap();
        let (pos_nxt, cost_mv) = pos.do_move(mv).unwrap();

        pos = pos_nxt;
        cost_total += cost_mv;

        println!("着手 {i}: {} (cost={cost_mv})", src.to_inner());
        println!("{pos}");
    }

    cost_total += COST_CLEAR_ERASE_BLOCK * pos.block_count() as Cost;

    println!("総コスト: {cost_total}");
}

fn format_fceux(pos: Position, moves: &[Move], solution: &Solution) {
    let inputs = solution_to_movie(pos, moves, solution);

    println!("TAS {}", inputs.len());

    for input in inputs {
        println!("{}", input.display_fceux());
    }
}

fn format_neshawk(pos: Position, moves: &[Move], solution: &Solution) {
    let inputs = solution_to_movie(pos, moves, solution);

    for input in inputs {
        println!("{}", input.display_neshawk());
    }
}

fn solution_to_movie(mut pos: Position, moves: &[Move], solution: &Solution) -> Vec<MovieInput> {
    let mut inputs = Vec::<MovieInput>::new();

    for &src in solution.moves().iter() {
        assert_ne!(pos.move_remain(), 0);

        // 自機を動かして待つ。
        let inputs_hero = inputs_hero_move(pos.hero_row(), src);
        inputs.extend(&inputs_hero);

        let mv = moves.iter().copied().find(|mv| mv.src() == src).unwrap();
        let (pos_nxt, cost_mv) = pos.do_move(mv).unwrap();

        // ブロックを投げて待つ。
        let wait_len = cost_mv as usize - inputs_hero.len() - 1;
        inputs.push(MovieInput::A);
        inputs.extend(vec![MovieInput::None; wait_len]);

        pos = pos_nxt;
    }

    inputs
}

fn inputs_hero_move(from: MoveSrc, to: MoveSrc) -> Vec<MovieInput> {
    use std::cmp::Ordering;

    const WAIT_LEN: usize = COST_HERO_STEP as usize - 1;

    let mut inputs = Vec::<MovieInput>::new();

    match from.cmp(&to) {
        Ordering::Less => {
            for _ in 0..to.to_inner() - from.to_inner() {
                inputs.push(MovieInput::Down);
                inputs.extend([MovieInput::None; WAIT_LEN]);
            }
        }
        Ordering::Greater => {
            for _ in 0..from.to_inner() - to.to_inner() {
                inputs.push(MovieInput::Up);
                inputs.extend([MovieInput::None; WAIT_LEN]);
            }
        }
        _ => {}
    }

    inputs
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MovieInput {
    None,
    A,
    Up,
    Down,
}

impl MovieInput {
    fn display_fceux(self) -> &'static str {
        match self {
            Self::None => "",
            Self::A => "A",
            Self::Up => "U",
            Self::Down => "D",
        }
    }

    fn display_neshawk(self) -> &'static str {
        match self {
            Self::None => "|..|........|........|",
            Self::A => "|..|.......A|........|",
            Self::Up => "|..|U.......|........|",
            Self::Down => "|..|.D......|........|",
        }
    }
}
