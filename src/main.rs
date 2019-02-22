#![deny(clippy::all)]
use frontend::{parse, render, Parse, Render};
use structopt::StructOpt;

mod frontend;

fn main() {
    let input = App::from_args();
    match input.subcmd {
        Command::Parse { input } => parse(input),
        Command::Render { input } => render(input),
    };
}

#[derive(Debug, StructOpt)]
struct App {
    #[structopt(subcommand, help = "The subcommand to run")]
    pub subcmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(
        name = "parse",
        about = "Parse some options into a serializable format"
    )]
    Parse {
        #[structopt(flatten)]
        input: Parse,
    },
    #[structopt(name = "render", about = "Render a timeline file")]
    Render {
        #[structopt(flatten)]
        input: Render,
    },
}
