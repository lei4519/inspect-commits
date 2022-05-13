use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus};
use std::str::from_utf8;
use std::{env, io};
use std::{ffi::OsStr, process::Stdio};

pub fn get_file(path: impl AsRef<Path> + Debug) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("文件不存在，初始化配置文件失败")
}

pub fn get_root_path() -> io::Result<PathBuf> {
    let file_path = env::current_exe()?;
    let mut file_path = PathBuf::from(exec_out_str("readlink", [file_path.to_str().unwrap()])?);

    file_path.pop();
    file_path.pop();

    Ok(file_path)
}

pub fn spawn<I, S>(c: &str, args: I) -> io::Result<Child>
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S> + Debug + Copy,
{
    Command::new(c).args(args).stdout(Stdio::piped()).spawn()
}

pub fn exec<I, S>(c: &str, args: I) -> io::Result<ExitStatus>
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S> + Debug + Copy,
{
    let mut c = spawn(c, args)?;
    c.wait()
}

pub fn exec_out_str<I, S>(c: &str, args: I) -> io::Result<String>
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S> + Debug + Copy,
{
    let output = spawn(c, args)?.wait_with_output()?;
    Ok(from_utf8(output.stdout.as_slice()).unwrap().to_string())
}

/// max_line: max line to emit
pub fn exec_out_call<I, S, F>(max_line: u32, c: &str, args: I, mut f: F) -> io::Result<()>
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S> + Debug + Copy,
    F: FnMut(&String) -> bool,
{
    let mut child_process = Command::new(c).args(args).stdout(Stdio::piped()).spawn()?;

    let mut out_buf = BufReader::new(child_process.stdout.as_mut().unwrap());
    let mut out_line = String::new();

    let mut count = 0;

    loop {
        if count == max_line {
            count = 0;
            out_line.clear();
        }

        if matches!(out_buf.read_line(&mut out_line), Ok(size) if size != 0) {
            count = count + 1;
            if count == max_line {
                if !f(&out_line) {
                    break;
                }
            }
        } else {
            if !out_line.is_empty() {
                f(&out_line);
            }
            break;
        }
    }
    Ok(())
}
