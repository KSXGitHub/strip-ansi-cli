use structopt::*;

#[derive(StructOpt)]
#[structopt(name = "strip-ansi")]
pub struct Args {
    /// Input text.
    /// Use stdin if not specified.
    #[structopt(name = "TEXT")]
    pub text: Option<String>,
}
