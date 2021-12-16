use crate::utils::{exec, exec_out_str, get_root_path};
use colored::*;

pub fn get_hookdir_path() -> String {
    let mut path = get_root_path();
    path.push("hooks");
    path.to_str().unwrap().to_string()
}

pub async fn set_global_hook() {
    let hook_path = get_hookdir_path();

    let prev_path = exec_out_str("git", ["config", "--global", "--get", "core.hooksPath"]).await;

    if !prev_path.is_empty() {
        if hook_path == prev_path.trim() {
            println!("{}", "配置成功".green());
            return;
        }
        println!(
            "{}",
            "存在全局 hooksPath，已覆盖配置。\n如果需要执行多个 hook，考虑使用 husky 库进行管理。"
                .yellow()
        );
    }

    exec("git", ["config", "--global", "core.hooksPath", &hook_path]).await;
    println!("{}\n{}", "注意：仓库中配置的 hooksPath 会覆盖全局行为，如果仓库中也有 hook 执行，考虑使用 husky 库进行管理。".yellow(), "配置成功".green());
}

pub async fn unset_global_hook() {
    let cur_path = exec_out_str("git", ["config", "--global", "--get", "core.hooksPath"]).await;

    let hook_path = get_hookdir_path();

    if cur_path.is_empty() || hook_path != cur_path.trim() {
        println!("{}", "配置已清除".green());
        return;
    }
    exec("git", ["config", "--global", "--unset", "core.hooksPath"]).await;
    println!("{}", "配置已清除".green());
}

// pub async fn pre_push() {
//     // TODO 执行自己的逻辑
//     // TODO 执行已存在的全局 hooksPath
//     // TODO 执行本地已存在的 hooksPath
//     // TODO 执行本地 .git 目录中的 hook
// }

// /** 执行之前设置的全局 hooksPath */
// async fn exec_global_hooks_path(hook_name: &str) {
//     let (conf, ..) = read_config().await;
//     if let Some(prev_path) = conf.previous_hook_path {
//         exec_hook(prev_path, hook_name);
//     }
// }

// /** 执行仓库的 hooksPath */
// async fn exec_local_hooks_path(hook_name: &str) {
//     let mut repo = exec_out_str("git", ["config", "--get", "core.hooksPath"]).await;
//     if !repo.is_empty() {
//         repo.push_str(hook_name);
//         exec_hook(repo, hook_name);
//     }
// }

// /** 执行 .git/hooks */
// async fn exec_git_hook(hook_name: &str) {
//     let mut repo = exec_out_str(
//         "git",
//         [
//             "rev-parse",
//             "--show-superproject-working-tree",
//             "--show-toplevel",
//         ],
//     )
//     .await;
//     repo.push_str("/.git/hooks/");
//     exec_hook(repo, hook_name);
// }

// /** 检查 hook 是否存在并执行 */
// async fn exec_hook(mut path: String, hook_name: &str) {
//     path.push_str(hook_name);
//     if Path::new(&path).exists() {
//         let a = exec(&path).await;
//         if matches!(a.code(), Some(code) if code != 0) {
//             exit(1);
//         }
//     }
// }
