use std::path::{Path, PathBuf};

use anyhow::{ensure, Context as _};
use clap::Parser;

use flipull_advance_solver::*;

/// 原作の ROM ファイルから ADVANCE モードの問題を抽出する。
#[derive(Debug, Parser)]
struct Cli {
    /// 原作の ROM ファイル (iNES 形式)。
    path_ines: PathBuf,

    /// 面 (1..=50)。
    #[arg(value_parser = clap::value_parser!(u8).range(1..=50))]
    stage: u8,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let rom = Rom::from_ines_file(&cli.path_ines)?;

    // 面を 0-based に直す。
    let stage = cli.stage - 1;

    let problem = extract(&rom, stage);

    print!("{problem}");

    Ok(())
}

fn extract(rom: &Rom, stage: u8) -> Problem {
    let (bank, ptrs_offset) = if stage < 25 {
        (rom.chr_bank(0), 0x0A00 + 4 * usize::from(stage))
    } else {
        (rom.chr_bank(2), 0x1A00 + 4 * usize::from(stage - 25))
    };

    let mut board = ProblemBoard::new();

    // ブロック配置、初期保持ブロック、残り手数を読み取る。
    let (block_holding, move_remain) = {
        let ptr = usize::from(read_u16_le(&bank[ptrs_offset..]) & 0x3FFF);
        let buf = &bank[ptr..][..48 + 2];

        for (row, col) in itertools::iproduct!(0..6, 0..8) {
            let block = buf[8 * row + col];
            let block = match block {
                0 => None,
                1 => Some(Block::Normal1),
                2 => Some(Block::Normal2),
                3 => Some(Block::Normal3),
                4 => Some(Block::Normal4),
                // ADVANCE モードでは盤面にワイルドカードが現れることはない。
                _ => panic!("無効な盤面ブロック値: {block}"),
            };
            let tile = block.map(ProblemTile::Block);
            board[(col, row + 6)] = tile;
        }

        let move_remain = buf[48];

        let block_holding = buf[49];
        let block_holding = Block::from_inner(block_holding)
            .unwrap_or_else(|| panic!("無効な保持ブロック値: {block_holding}"));

        (block_holding, move_remain)
    };

    // 壁/パイプの配置を読み取る。
    {
        let ptr = usize::from(read_u16_le(&bank[ptrs_offset + 2..]) & 0x3FFF);
        let buf = &bank[ptr..][..12 * 2];

        for (row, &value) in buf[..12].iter().enumerate() {
            for col in 0..8 {
                if (value & (1 << (7 - col))) != 0 {
                    board[(col, row)] = Some(ProblemTile::Wall);
                }
            }
        }

        for (row, &value) in buf[12..].iter().enumerate() {
            for col in 0..8 {
                if (value & (1 << (7 - col))) != 0 {
                    board[(col, row)] = Some(ProblemTile::Pipe);
                }
            }
        }
    }

    Problem::new(board, block_holding, move_remain)
        .expect("問題が ADVANCE モードの制約を満たしていない")
}

fn read_u16_le(buf: &[u8]) -> u16 {
    let buf: [u8; 2] = buf[..2].try_into().unwrap();
    u16::from_le_bytes(buf)
}

const PRG_LEN: usize = 0x8000;

const CHR_BANK_COUNT: usize = 4;
const CHR_BANK_LEN: usize = 0x2000;
const CHR_LEN: usize = CHR_BANK_LEN * CHR_BANK_COUNT;

#[derive(Debug)]
struct Rom {
    _prg: Box<[u8; PRG_LEN]>,
    chr: Box<[u8; CHR_LEN]>,
}

impl Rom {
    fn from_ines_file(path: &Path) -> anyhow::Result<Self> {
        const HEADER_LEN: usize = 16;

        let ines = std::fs::read(path)
            .with_context(|| format!("ROM ファイル '{}' を読めない", path.display()))?;

        ensure!(ines.len() >= HEADER_LEN, "iNES ヘッダの途中で EOF に達した");
        let (header, body) = ines.split_at(HEADER_LEN);

        ensure!(header.starts_with(b"NES\x1A"), "iNES magic がない");

        ensure!(body.len() >= PRG_LEN, "PRG の途中で EOF に達した");
        let (_prg, chr) = body.split_at(PRG_LEN);
        ensure!(
            chr.len() == CHR_LEN,
            "CHR サイズが一致しない (expect={CHR_LEN:#06X}, actual={:#06X})",
            chr.len()
        );

        let _prg: Box<[u8; PRG_LEN]> = _prg.to_vec().try_into().unwrap();
        let chr: Box<[u8; CHR_LEN]> = chr.to_vec().try_into().unwrap();

        Ok(Self { _prg, chr })
    }

    fn chr_bank(&self, id: usize) -> &[u8; CHR_BANK_LEN] {
        self.chr[CHR_BANK_LEN * id..][..CHR_BANK_LEN]
            .try_into()
            .unwrap()
    }
}
