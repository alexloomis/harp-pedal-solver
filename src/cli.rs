use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use lazy_static::lazy_static;
use std::path::PathBuf;

const SHOW: usize = 3;
const PEDAL_COST: usize = 1000;
// const DOUBLE_CHANGE_COST: usize = 300;
const DOUBLE_STRING_COST: usize = 100;
const CROSS_STRING_COST: usize = 1200;
const EARLY_CHANGE_COST: usize = 300;
const QUICK_CHANGE_COST: usize = 30;
const QUICK_CHANGE_DECAY: usize = 10;
const PEDAL_DISTANCE_COST: usize = 1;
// How much do we penalize each beat that
// a string is different than the key signature?
#[allow(dead_code)]
const OUT_OF_KEY_COST: usize = 0;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    pub file: PathBuf,
    /// Write output to FILE.
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
    /// Limit how many possibilities are shown. To show all, set show = 0.
    #[arg(long, default_value_t = SHOW, value_name = "INT")]
    pub show: usize,
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
    /// The cost for each pedal change.
    #[arg(long, default_value_t = PEDAL_COST, value_name = "INT")]
    pub pedal_cost: usize,
    /// How much we penalize simultaneous pedal changes.
    // #[arg(long, default_value_t = DOUBLE_CHANGE_COST, value_name = "INT")]
    // pub double_change_cost: usize,
    /// How much we penalize doubled strings (eg E# and F).
    #[arg(long, default_value_t = DOUBLE_STRING_COST, value_name = "INT")]
    pub double_string_cost: usize,
    /// How much we penalize crossed strings (eg E# and Fb).
    #[arg(long, default_value_t = CROSS_STRING_COST, value_name = "INT")]
    pub cross_string_cost: usize,
    /// How much we penalize pedaling early.
    #[arg(long, default_value_t = EARLY_CHANGE_COST, value_name = "INT")]
    pub early_change_cost: usize,
    /// How much we penalize successive changes.
    #[arg(long, default_value_t = QUICK_CHANGE_COST, value_name = "INT")]
    pub quick_change_cost: usize,
    /// How much quick-change-cost decays each beat without a change.
    #[arg(long, default_value_t = QUICK_CHANGE_DECAY, value_name = "INT")]
    pub quick_change_decay: usize,
    /// How much we penalize distance between pedals for successive changes.
    #[arg(long, default_value_t = PEDAL_DISTANCE_COST, value_name = "INT")]
    pub pedal_diatance_cost: usize,
}

lazy_static! {
    pub static ref CLI: Cli = Cli::parse();
}
