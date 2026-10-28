use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mori", version, about = "Disk usage tree viewer")]
pub struct Args {
    #[arg(default_value = ".", help = "Directory to scan")]
    pub path: PathBuf,

    #[arg(
        short,
        long,
        default_value_t = 3,
        value_name = "N",
        help = "How many levels of subdirectories to show"
    )]
    pub depth: usize,

    #[arg(
        short,
        long,
        default_value_t = 20,
        value_name = "N",
        help = "Max entries per directory, the rest are grouped"
    )]
    pub limit: usize,

    #[arg(short = 'H', long, help = "Include hidden files and directories")]
    pub hidden: bool,
}