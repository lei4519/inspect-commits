use crate::config::read_config;
use crate::utils::{exec_out_call, exec_out_str, spawn};
use colored::*;
use regex::Regex;
use std::io;
use std::process::exit;

// git push <远程主机名> <本地分支名>:<远程分支名>
/// 检查 commit 中的敏感字
/// `remote_repo_name` 远程仓库的名称, e.g. `origin`  `upstream`
/// `remote_branch_name` 远程分支的名称
/// `local_branch_name` 本地分支的名称, e.g. `origin`  `upstream`
pub fn check(
    remote_repo_name: Option<&str>,
    remote_branch_name: Option<&str>,
    local_branch_name: Option<&str>,
) -> io::Result<()> {
    println!("{}", "🚀 Validate Git Push running...".cyan());

    let mut c = spawn("git", ["rev-parse"])?;
    let status = c.wait()?;

    if matches!(status.code(), Some(code) if code != 0) {
        println!("{}", "cwd is not a git repo".red());
        exit(0);
    }

    let (config, ..) = read_config()?;

    // 空的 config
    if !config.rules.as_ref().map_or(false, |rules|
        // 所有的rule 都没有配置 words
        rules.iter().any(|rule| !rule.words.is_empty()))
    {
        println!(
            "{}",
            "No rules have been configured，please use inspect-commits config".yellow()
        );
        exit(0);
    }

    let mut remote_url = String::new();

    if remote_repo_name.is_some() {
        remote_url = exec_out_str(
            "git",
            [
                "config",
                "--get",
                format!("remote.{}.url", remote_repo_name.unwrap()).as_str(),
            ],
        )?;
        if remote_url.is_empty() {
            println!(
                "Remote repo is not exist, please check remote repo name：{}",
                remote_repo_name.unwrap()
            );
            exit(0)
        }
    }

    let arg = if remote_repo_name.is_none() {
        // 没有 remote_name 说明要检查所有的 commits
        println!("🔎 {}", "Check all commit".cyan());
        "--all".to_string()
    } else {
        let (local_branch_name, remote_branch_name) =
            if local_branch_name.is_some() && remote_branch_name.is_some() {
                // pre-push shell params
                // refs/heads/master refs/heads/foreign
                let mut local = local_branch_name.unwrap().to_string();
                let mut remote = remote_branch_name.unwrap().to_string();
                if local.len() > 11 && remote.len() > 11 {
                    let ref_str = "refs/heads/";
                    if &local[0..11] == ref_str && &remote[0..11] == ref_str {
                        local = local[11..].to_string();
                        remote = remote[11..].to_string();
                    }
                }
                (local, remote)
            } else {
                let local = local_branch_name.map_or_else(
                    || {
                        exec_out_str("git", ["branch", "--show-current"])
                            .expect("get current branch failed")
                    },
                    |v| v.to_string(),
                );
                let local = local.trim().to_string();
                let remote = remote_branch_name.map_or_else(|| local.clone(), |v| v.to_string());
                (local, remote)
            };

        let v = format!(
            "{}/{}..{}",
            remote_repo_name.unwrap_or("origin"),
            remote_branch_name,
            local_branch_name
        );

        println!("🔎 Check {}", &v.cyan());

        v
    };

    let commits = exec_out_str("git", ["log", "--pretty=format:%h", arg.as_str()])?;

    if commits.trim().is_empty() {
        println!("The current branch has no commits that are not pushed to the remote repository");
        exit(0)
    }

    let commits = commits
        .split("\n")
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    let rules = config.rules.as_ref().unwrap();

    let words_str = rules
        .iter()
        .fold(vec![], |mut ws, rule| {
            if rule
                .exclude_repo_urls
                .iter()
                .any(|url| remote_url.contains(url))
            {
                println!("Hit exclude URL: {}, Words: {:?}", remote_url.trim(), rule.words);
                return ws;
            }

            let mut s = rule.words.iter().fold(String::from(""), |mut acc, word| {
                if word.is_empty() {
                    return acc;
                }
                acc.push_str(word);
                acc.push('|');
                return acc;
            });
            s.pop();
            ws.push(s);
            return ws;
        })
        .join("|");

    if words_str.is_empty() {
        println!("{}", "There are no more words to check".cyan());
        exit(0);
    } else {
        println!("{}: {}", "Sensitive words".cyan(), words_str.cyan())
    }


    let words_reg =
        Regex::new(&words_str).expect("There are characters in the word that cannot build the regular expression.");

    for commit in commits {
        exec_out_call(5, "git", ["show", &commit, "--pretty=format:%s"], |line| {
            if let Some(cap) = words_reg.captures(line) {
                let word = cap.get(0).map_or("", |m| m.as_str());
                println!(
                    "{} {}\n{} {}\n{}\n{}{}",
                    "💥 Word:".red().bold(),
                    word,
                    "💥 Commit:".red().bold(),
                    commit,
                    "💥 Content:".red().bold(),
                    line,
                    line.replace(word, &format!("\x1b[41;37m{}\x1b[0m", word))
                        .on_bright_black()
                );
                exit(1);
            }

            true
        })?;

        println!("✨ {}", commit.green());
    }
    println!("🎉 {}", "Commits is secure".green(),);
    Ok(())
}
