use clap::Parser;

#[derive(Parser, Debug)]
pub struct HistoryCmd {
    show_history: bool
}