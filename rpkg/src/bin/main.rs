use clap::Parser;
use rpkg::pkg;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = None,
)]
pub struct Args {
    /// .pkg path
    path: String,
}

fn main() {
    let args = Args::parse();
    let query_pkg = &args.path;

    for asset in pkg::match_file(query_pkg) {
        println!("{asset}");
    }
}
