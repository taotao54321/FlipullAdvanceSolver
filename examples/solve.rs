use std::path::PathBuf;

use anyhow::{ensure, Context as _};
use clap::Parser;
use log::info;

use flipull_advance_solver::*;

/// 指定した問題に対する実時間最速の解を求める。
#[derive(Debug, Parser)]
struct Cli {
    /// 最終面かどうか。
    #[arg(long)]
    last_stage: bool,

    /// 問題ファイル。
    path_problem: PathBuf,
}

fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cli = Cli::parse();

    let problem = std::fs::read_to_string(&cli.path_problem).with_context(|| {
        format!(
            "問題ファイル '{}' を読み取れない",
            cli.path_problem.display()
        )
    })?;
    let problem: Problem = problem.parse()?;

    if let Some((solution, cost)) = solve_problem(&problem, cli.last_stage) {
        println!("{solution}");

        let cost_verify = solution
            .verify(&problem, cli.last_stage)
            .context("最適解の verify に失敗")?;
        ensure!(
            cost_verify == cost,
            "最適解の verify に失敗: コストが一致しない (solve: {cost}, verify: {cost_verify})"
        );
    } else {
        info!("NO SOLUTION FOUND");
    }

    Ok(())
}
