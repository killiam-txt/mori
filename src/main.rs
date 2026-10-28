mod cli;
mod render;
mod scan;
mod size;
mod tree;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    if !args.path.is_dir() {
        eprintln!("mori: {} is not a directory", args.path.display());
        std::process::exit(1);
    }

    let root = scan::build(&args.path, args.hidden);
    render::print(&root, args.depth, args.limit);
}