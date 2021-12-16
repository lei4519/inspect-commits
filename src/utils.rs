use std::env;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::process::{ExitStatus};
use std::str::from_utf8;
use std::{ffi::OsStr, process::Stdio};
use tokio::fs::{File, OpenOptions};
use tokio::process::Child;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

pub async fn get_file(path: impl AsRef<Path>) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .await
        .expect("文件不存在，初始化配置文件失败")
}

pub fn get_root_path() -> PathBuf {
    let mut file_path = env::current_exe().unwrap();
    file_path.pop();
    file_path.pop();
    file_path.pop();
    file_path
}

pub async fn spawn<I, S>(c: &str, args: I) -> Child
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S> + Debug + Copy,
{
    Command::new(c)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect(format!("failed to {} {:?}", c, args).as_str())
}

pub async fn exec<I, S>(c: &str, args: I) -> ExitStatus
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S> + Debug + Copy,
{
    let mut c = spawn(c, args).await;
    let s = c.wait().await.unwrap();
    s
}

pub async fn exec_out_str<I, S>(c: &str, args: I) -> String
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S> + Debug + Copy,
{
    let c = spawn(c, args).await;
    let output = c.wait_with_output().await.unwrap();
    from_utf8(output.stdout.as_slice()).unwrap().to_string()
}

pub async fn exec_out_call<I, S, F>(c: &str, args: I, f: F)
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S> + Debug + Copy,
    F: Fn(&String) -> bool,
{
    let mut child_process = Command::new(c)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect(format!("failed to {} {:?}", c, args).as_str());

    let mut out_buf = BufReader::new(child_process.stdout.as_mut().unwrap());
    let mut out_line = String::new();
    // buf max line to emit
    let mut count = 0;
    let max = 10;

    loop {
        if count == max {
            count = 0;
            out_line.clear();
        }

        if matches!(out_buf.read_line(&mut out_line).await, Ok(size) if size != 0) {
            count = count + 1;
            if count == max && !f(&out_line) {
                break;
            }
        } else {
            break;
        }
    }
}
