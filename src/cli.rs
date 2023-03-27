use clap::Parser;
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
    #[arg(long, default_value_t = SHOW, value_name = "INT")]
    /// Limit how many possibilities are shown. To show all, set show = 0.
    pub show: usize,
    #[arg(long, default_value_t = PEDAL_COST, value_name = "INT")]
    /// UNIMPLEMENTED.
    /// The cost for each pedal change.
    pub pedal_cost: usize,
    #[arg(long, default_value_t = DOUBLE_CHANGE_COST, value_name = "INT")]
    /// UNIMPLEMENTED.
    /// How much we penalize simultaneous pedal changes.
    pub double_change_cost: usize,
    #[arg(long, default_value_t = DOUBLE_STRING_COST, value_name = "INT")]
    /// UNIMPLEMENTED.
    /// How much we penalize doubled strings (eg E# and F).
    pub double_string_cost: usize,
    #[arg(long, default_value_t = CROSS_STRING_COST, value_name = "INT")]
    /// UNIMPLEMENTED.
    /// How much we penalize crossed strings (eg E# and Fb).
    pub cross_string_cost: usize,
}
