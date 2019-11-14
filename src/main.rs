extern crate serde_json;
extern crate structopt;

mod line_pair;
mod util_fn;
mod warpy;

use structopt::StructOpt;
use util_fn::make_morphy;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path_to_first_image: std::path::PathBuf,

    #[structopt(parse(from_os_str))]
    path_to_second_image: std::path::PathBuf,

    #[structopt(parse(from_os_str))]
    path_to_json: std::path::PathBuf,

    #[structopt(parse(from_os_str))]
    path_to_output: std::path::PathBuf,
}

fn main() {
    let cli = Cli::from_args();
    make_morphy(
        &cli.path_to_first_image,
        &cli.path_to_second_image,
        &cli.path_to_json,
        &cli.path_to_output,
    )
}
