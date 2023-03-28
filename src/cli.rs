use clap::Parser;
use lazy_static::lazy_static;
use std::path::PathBuf;

const PEDAL_COST: usize = 100;
const DOUBLE_CHANGE_COST: usize = 40;
const DOUBLE_STRING_COST: usize = 10;
const CROSS_STRING_COST: usize = 120;
// How much do we penalize each beat that
// a string is different than the key signature?
#[allow(dead_code)]
const OUT_OF_KEY: usize = 0;
const SHOW: usize = 3;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    pub file: PathBuf,
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(long)]
    pub debug: bool,
    /// Limit how many possibilities are shown. To show all, set show = 0.
    #[arg(long, default_value_t = SHOW, value_name = "INT")]
    pub show: usize,
    /// The cost for each pedal change.
    #[arg(long, default_value_t = PEDAL_COST, value_name = "INT")]
    pub pedal_cost: usize,
    /// How much we penalize simultaneous pedal changes.
    #[arg(long, default_value_t = DOUBLE_CHANGE_COST, value_name = "INT")]
    pub double_change_cost: usize,
    /// How much we penalize doubled strings (eg E# and F).
    #[arg(long, default_value_t = DOUBLE_STRING_COST, value_name = "INT")]
    pub double_string_cost: usize,
    /// How much we penalize crossed strings (eg E# and Fb).
    #[arg(long, default_value_t = CROSS_STRING_COST, value_name = "INT")]
    pub cross_string_cost: usize,
}

lazy_static! {
    pub static ref CLI: Cli = Cli::parse();
}
