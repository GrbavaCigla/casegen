use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "casegen",
    about = "Generate test cases for competitive programming tasks."
)]
pub struct Opt {
    #[structopt(help="Case file")]
    pub filepath: PathBuf,
}
