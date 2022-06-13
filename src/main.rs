/*
git config --unset core.hooksPath
git config --get core.hooksPath
git config core.hooksPath <path>
git rev-parse --show-superproject-working-tree --show-toplevel
git log --pretty=format:%H origin..HEAD
git show --pretty=format:%b hash
*/

mod check;
mod cli;
mod config;
mod hook;
mod utils;

use cli::build_cli;
use std::{env, io, process::Command};

use check::check;
use config::get_config_path;
use hook::{set_global_hook, unset_global_hook};

fn main() -> io::Result<()> {
    let mut cmd = build_cli();

    let matches = cmd.clone().get_matches();

    match matches.subcommand() {
        Some(("check", sub_m)) => {
            let remote_repo_name = sub_m.value_of("REMOTE_REPO_NAME");
            let remote_branch_name = sub_m.value_of("REMOTE_BRANCH_NAME");
            let local_branch_name = sub_m.value_of("LOCAL_BRANCH_NAME");
            check(remote_repo_name, remote_branch_name, local_branch_name)?;
        }
        Some(("checkall", _)) => {
            check(None, None, None)?;
        }
        Some(("config", sub_m)) => {
            let path = get_config_path();
            let path = path.to_str().unwrap();
            if sub_m.is_present("PATH") {
                println!("{}", path);
            } else {
                let editor = env::var_os("EDITOR").unwrap_or("vi".into());
                Command::new(editor)
                    .arg(path)
                    .spawn()
                    .expect(&format!(
                        "open editor failed, please manual edit config: {}",
                        path
                    ))
                    .wait()?;
            }
        }
        Some(("set-global-hook", _)) => {
            set_global_hook()?;
        }
        Some(("unset-global-hook", _)) => {
            unset_global_hook()?;
        }
        _ => {
            cmd.print_help().unwrap();
        }
    }
    Ok(())
}
