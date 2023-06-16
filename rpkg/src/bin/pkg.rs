use clap::Parser;

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
    // let args = Args::parse();
    // let query_pkg = &args.path;

    // NOTE: 变更 .pkg 格式后原匹配功能暂时废弃
    todo!();
}
