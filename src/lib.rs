pub mod app;
pub mod args;
pub mod sub;

pub fn main() {
    use pipe_trait::*;
    use structopt_utilities::StructOptUtils;
    args::Args::strict_from_args()
        .pipe(app::App::from_args)
        .exec()
}

pub use structopt_utilities::{self, clap, structopt};
