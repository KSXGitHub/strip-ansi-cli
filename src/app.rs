use super::args::Args;
use pipe_trait::*;
use std::io::{stdin, stdout, Read, Write};

pub struct App {
    pub input: String,
}

impl App {
    pub fn from_args(args: Args) -> Self {
        let Args { text } = args;
        let input = if let Some(input) = text {
            input
        } else {
            let mut buf = String::new();
            stdin().read_to_string(&mut buf).expect("read from stdin");
            buf
        };
        App { input }
    }

    pub fn exec(self) {
        self.input
            .pipe(strip_ansi_escapes::strip)
            .expect("strip ansi symbols")
            .pipe(|output| stdout().write(&output))
            .expect("write to stdout");
    }
}
