use clap::Parser;


#[derive(Parser, Debug)]
pub struct OpenCmd {
    /// The file to open
    pub open: bool
}