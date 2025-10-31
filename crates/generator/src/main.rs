use std::{path::PathBuf, str::FromStr};

use anyhow::{Context, Result, anyhow};
use clap::{
    Parser,
    builder::{PossibleValuesParser, TypedValueParser},
};
use sudoku_gen::{Board, BoardSize, Difficulty, IrregularMap, RegionKind};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[arg(
        short,
        long,
        default_value_t = 9,
        value_parser = PossibleValuesParser::new(["4", "6", "9", "12", "16"]).map(
            // safe to unwrap() because input s is constrained to known good values
            |s| s.parse::<u8>().unwrap()
        ),
    )]
    /// The maximum number of cells in any column or row.
    ///
    /// Ignored if importing a puzzle from a file or string.
    size: u8,

    #[arg(short,
        long,
        alias = "jigsaw",
        action = clap::ArgAction::SetTrue,
        conflicts_with = "export",
    )]
    /// Use an irregular (jigsaw) regions instead of the regular rectangular regions.
    ///
    /// Ignored if importing a puzzle from a file or string.
    irregular: bool,

    #[arg(
        short,
        long,
        action = clap::ArgAction::SetTrue,
        overrides_with = "irregular",
        verbatim_doc_comment,
    )]
    /// Export the solved board as a string instead of pretty printing it.
    ///
    /// This turns off other console output except for errors.
    /// This also forces regular regions (nullifies `--irregular`).
    export: bool,

    #[arg(short = 'p', long, verbatim_doc_comment)]
    /// Load a board from a file instead of generating a new one.
    ///
    /// Ignored if importing a puzzle from a string.
    /// See expectations about the optional argument STRING.
    from_path: Option<PathBuf>,

    #[arg(name = "STRING", verbatim_doc_comment)]
    /// Load a board from a string instead of generating a new one.
    ///
    /// Line endings are stripped before parsing.
    /// The board's size is determined by the length of the
    /// string after stripping.
    ///
    /// Filled cells are represented using
    ///   1-4 for 4x4 boards
    ///   1-6 for 6x6 boards
    ///   1-9 for 9x9 boards
    ///   0-9 and A-B (or a-b) for 12x12 boards
    ///   0-9 and A-F (or a-f) for 16x16 boards
    /// Empty cells are represented by any other character
    /// (including spaces and commas).
    from_str: Option<String>,

    #[arg(short, long)]
    /// Reduce a solution into a puzzle of the specified difficulty.
    ///
    /// Ignored when importing a puzzle from a file or string.
    difficulty: Option<Difficulty>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut board = if let Some(s) = cli.from_str {
        let board = Board::from_str(&s)?;
        if !cli.export {
            print!("Imported:\n{board}");
        }
        let result = board.validate(false);
        if !result.is_ok() {
            eprintln!(
                "The imported board is invalid!\n\
                Note: ValidationResults use zero-based indexing\n\
                {result:#?}"
            );
            return Err(anyhow!("The imported board is invalid."));
        }
        if !cli.export {
            println!("The imported board is seems valid.");
        }
        board
    } else if let Some(path) = cli.from_path {
        let content =
            std::fs::read_to_string(path).with_context(|| "Failed to read puzzle from file")?;
        let board = Board::from_str(&content)?;
        if !cli.export {
            print!("Imported:\n{board}");
        }
        let result = board.validate(false);
        if !result.is_ok() {
            eprintln!(
                "The imported board is invalid!\n\
                Note: ValidationResults use zero-based indexing\n\
                {result:#?}"
            );
            return Err(anyhow!("The imported board is invalid."));
        }
        if !cli.export {
            println!("The imported board is seems valid.");
        }
        board
    } else {
        let size = BoardSize::from(cli.size);
        let max_cells = size.max_cells();
        if !cli.export {
            println!(
                "Generating a {} Sudoku board of size {max_cells}x{max_cells}...",
                if cli.irregular {
                    "irregular"
                } else {
                    "regular"
                }
            );
        }
        let kind = if cli.irregular {
            // if max_cells > 9 {
            //     eprintln!("Preventing an infinite loop");
            //     return Err(anyhow!(
            //         "Irregular regions are only supported for 4x4, 6x6, and 9x9 boards."
            //     ));
            // }
            let map = IrregularMap::generate(&size)
                .with_context(|| "Failed to generate an irregular region map")?;
            RegionKind::Irregular { map }
        } else {
            RegionKind::Regular
        };
        let mut board = Board::new(&size, kind);
        // short-circuit solving by seeding the first region
        board.seed();
        board
    };
    if !cli.export {
        println!("Solving the board...");
    }
    if board.solve() {
        if cli.export {
            println!("{}", String::from(&board));
        } else {
            print!("Solved:\n{board}");
        }
        let result = board.validate(true);
        if result.is_ok() {
            if !cli.export {
                println!("The board is valid.");
            }
            if let Some(difficulty) = cli.difficulty {
                if !cli.export {
                    println!(
                        "Reducing the board to a{} {difficulty:?} puzzle...",
                        if matches!(difficulty, Difficulty::Easy | Difficulty::Expert) {
                            "n"
                        } else {
                            ""
                        }
                    );
                }
                let puzzle = difficulty.make_puzzle(&board);
                if cli.export {
                    println!("{}", String::from(&puzzle));
                } else {
                    print!("Puzzle ({difficulty:?}):\n{puzzle}");
                }
            }
        } else {
            eprintln!("{result:#?}");
            return Err(anyhow!("The board is invalid."));
        }
        Ok(())
    } else {
        Err(anyhow!("Failed to solve the board."))
    }
}
