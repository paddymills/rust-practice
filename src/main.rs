use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "branch", about = "branch template")]
struct Opt {
    name: String,
}

fn main() {
    let opt = Opt::from_args();

    run("git checkout template");
    run(&("git branch ".to_owned() + opt.name.as_str()));
    run(&("git checkout ".to_owned() + opt.name.as_str()));
}

fn run(args: &str) {
    let (shell, c) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    Command::new(shell)
        .arg(c)
        .arg(args)
        .output()
        .expect("failed to execute process");
}
