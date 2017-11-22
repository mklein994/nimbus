//#![cfg_attr(feature = "clippy", feature(plugin))]

#[macro_use]
extern crate clap;

use clap::Shell;

include!("src/args.rs");

fn main() {
    let outdir = match ::std::env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    println!("{:?}", outdir);

    let mut app = build_cli();
    app.gen_completions(crate_name!(), Shell::Bash, &outdir);
    app.gen_completions(crate_name!(), Shell::Fish, &outdir);
    app.gen_completions(crate_name!(), Shell::Zsh, &outdir);
    app.gen_completions(crate_name!(), Shell::PowerShell, &outdir);
}
