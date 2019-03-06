use crossbeam;
use std::{
    cmp,
    env::{self, current_dir},
    fs::{self, copy},
    path::Path,
    process::{Command, Stdio},
};

pub fn format_num(num: f64) -> String {
    let string = num.to_string();
    match num as i64 {
        0...1 => {
            let out = "  ".to_string() + &string[..1] + " KB";
            return out;
        }
        10...100 => {
            let out = "  ".to_string() + &string[..2] + " KB";
            return out;
        }
        100...1000 => {
            let out = "  ".to_string() + &string[..3] + " KB";
            return out;
        }
        1000...10000 => {
            let out = "".to_string() + &string[..1] + "," + &string[1..4] + " KB";
            return out;
        }
        10000...100000 => {
            let out = "  ".to_string() + &string[..2] + " MB";
            return out;
        }
        100000...1000000 => {
            let out = "  ".to_string() + &string[..3] + " MB";
            return out;
        }
        100000...10000000 => {
            let out = "".to_string() + &string[..1] + "," + &string[1..4] + " MB";
            return out;
        }
        10000000...100000000 => {
            let out = "  ".to_string() + &string[..2] + " GB";
            return out;
        }
        100000000...1000000000 => {
            let out = "  ".to_string() + &string[..3] + " GB";
            return out;
        }
        _ => {
            let out = string + " KB";
            return out;
        }
    }
}

pub fn delete_fs(path: String) -> std::io::Result<()> {
    fs::remove_file(path)?;
    Ok(())
}

pub fn strip(path: &str, orig_size: f64) -> () {
    let strip_path_string = path.to_string() + "strip";
    let strip_path_string_clone = strip_path_string.clone();
    copy(path, strip_path_string).expect("Error copying file");
    let strip_path = Path::new(&strip_path_string_clone);

    let strip = Command::new("strip")
        .arg("-s")
        .arg(strip_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to execute process");

    let output = strip.wait_with_output().expect("failed to wait on child");

    if output.status.success() {
        let strip_size = fs::metadata(strip_path).unwrap().len();
        let strip_size = strip_size as f64;
        let strip_diff_perc = (&strip_size / &orig_size) * 100 as f64;
        let strip_ratio = strip_diff_perc.to_string()[0..5].to_string() + "%";
        delete_fs(strip_path_string_clone).expect("Error removing Stripped file");
        let strip_file = format_num(strip_size / 1024 as f64);

        return println!("{0: <9}{1: <11}{2: <6}", "Strip", strip_file, strip_ratio);
    } else {
        delete_fs(strip_path_string_clone).expect("Error removing Stripped file");
        let error = String::from_utf8_lossy(&output.stderr);

        return println!("{}", error);
    }
}

pub fn striupx(path: &str, orig_size: f64, opt: &str) -> () {
    let strip_path_string = path.to_string() + "strip";
    let strip_path_string_clone = strip_path_string.clone();
    copy(path, strip_path_string).expect("Error copying file");
    let strip_path = Path::new(&strip_path_string_clone);

    let strip = Command::new("strip")
        .arg("-s")
        .arg(strip_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to execute process");

    let output = strip.wait_with_output().expect("failed to wait on child");

    if output.status.success() {
        let strip_size = fs::metadata(strip_path).unwrap().len();
        let strip_size = strip_size as f64;
        let strip_diff_perc = (&strip_size / &orig_size) * 100 as f64;
        let strip_ratio = strip_diff_perc.to_string()[0..5].to_string() + "%";
        let strip_file = format_num(strip_size / 1024 as f64);
        let option = "-".to_string() + opt;

        let upx = Command::new("upx")
            .arg(option)
            .arg(strip_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to execute process");

        let output = upx.wait_with_output().expect("failed to wait on child");

        if output.status.success() {
            let new_size = fs::metadata(strip_path).unwrap().len();
            let upx_size = new_size as f64;
            let upx_diff_perc = (&upx_size / &orig_size) * 100 as f64;
            let upx_ratio = upx_diff_perc.to_string()[0..5].to_string() + "%";
            delete_fs(strip_path_string_clone).expect("Error removing UPXed file");
            let upx_file = format_num(upx_size / 1024 as f64);

            return println!(
                "{0: <9}{1: <11}{2: <6}\n{3: <9}{4: <11}{5: <6}",
                "Both", upx_file, upx_ratio, "Strip", strip_file, strip_ratio
            );
        } else {
            delete_fs(strip_path_string_clone).expect("Error removing UPXed file");
            let error = String::from_utf8_lossy(&output.stderr);

            return print!(
                "{0: <9}{1: <11}{2: <6}\n{3}",
                "Strip", strip_file, strip_ratio, error
            );
        }
    } else {
        delete_fs(strip_path_string_clone).expect("Error removing Stripped file");
        let error = String::from_utf8_lossy(&output.stderr);

        return println!("{}", error);
    }
}

pub fn upx(path: &str, orig_size: f64, opt: &str) -> () {
    let upx_path_string = path.to_string() + "upx";
    let upx_path_string_clone = upx_path_string.clone();
    copy(path, upx_path_string).expect("Error copying file");
    let upx_path = Path::new(&upx_path_string_clone);
    let option = "-".to_string() + opt;

    let upx = Command::new("upx")
        .arg(option)
        .arg(upx_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to execute process");

    let output = upx.wait_with_output().expect("failed to wait on child");

    if output.status.success() {
        let new_size = fs::metadata(upx_path).unwrap().len();
        let upx_size = new_size as f64;
        let upx_diff_perc = (&upx_size / &orig_size) * 100 as f64;
        let upx_ratio = upx_diff_perc.to_string()[0..5].to_string() + "%";
        delete_fs(upx_path_string_clone).expect("Error removing UPXed file");
        let upx_file = format_num(upx_size / 1024 as f64);

        return println!("{0: <9}{1: <11}{2: <6}", "UPX", upx_file, upx_ratio);
    } else {
        delete_fs(upx_path_string_clone).expect("Error removing UPXed file");
        let error = String::from_utf8_lossy(&output.stderr);

        return print!("{}\r", error);
    }
}

pub fn is_executable<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_executable()
}

pub trait IsExecutable {
    // Returns `true` if there is a file at the given path and it is
    // executable. Returns `false` otherwise.
    fn is_executable(&self) -> bool;
}

impl IsExecutable for Path {
    fn is_executable(&self) -> bool {
        fs::metadata(self)
            .ok()
            .map_or(false, |meta| meta.permissions().is_executable())
    }
}

impl IsExecutable for fs::Permissions {
    fn is_executable(&self) -> bool {
        use std::os::unix::fs::PermissionsExt;
        self.mode() & 0o111 != 0
    }
}

pub fn can_strip() -> bool {
    let strip = Path::new("/usr/bin/strip");
    if path_exists(strip) {
        if strip.is_executable() {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

pub fn can_upx() -> bool {
    let strip = Path::new("/usr/bin/upx");
    if path_exists(strip) {
        if strip.is_executable() {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

pub fn get_path(str: &str) -> String {
    if str.starts_with("/") {
        // Determine if you're being given absolute path
        let path = str.to_string();
        return path;
    } else if str.starts_with("~") {
        // Interpret Linux home directory
        let user = Command::new("whoami")
            .output()
            .expect("failed to execute process");
        let user = String::from_utf8_lossy(&user.stdout);
        let path = "/home/".to_string() + &user.trim() + "/" + &str[1..];
        return path;
    } else {
        // Get current directory
        let wd = current_dir().unwrap();
        let pwd: String = wd.as_os_str().to_str().unwrap().to_string();
        let new_path = pwd + "/" + str;
        return new_path;
    }
}

pub fn mk_files(path: String, orig_size: f64, opt: &str) {
    if can_strip() == true && can_upx() == true {
        let orig_string = format_num(orig_size / 1024 as f64);
        println!("{0: <9}{1: <10} {2: <6}", "Method", "File Size", "Ratio");
        println!("------   ---------  ------");
        println!(
            "{0: <9}{1: <11}{2: <6}",
            &"Initial", &orig_string, &"100.0%"
        );

        // Run stip-upx combo same time as upx
        crossbeam::thread::scope(|scope| {
            let striupx_thread = scope.spawn(|| striupx(&path, orig_size, opt));
            let upx_thread = scope.spawn(|| upx(&path, orig_size, opt));
            cmp::max(striupx_thread.join().unwrap(), upx_thread.join().unwrap())
        });
    } else if can_strip() == true && can_upx() == false {
        let orig_string = format_num(orig_size / 1024 as f64);
        println!("{0: <9}{1: <10}{2: <6}", "Method", "File Size", "Ratio");
        println!("------   ---------  ------");
        println!(
            "{0: <9}{1: <11}{2: <6}",
            &"Initial", &orig_string, &"100.0%"
        );
        strip(&path, orig_size);
    } else if can_strip() == false && can_upx() == true {
        let orig_string = format_num(orig_size / 1024 as f64);
        println!("{0: <9}{1: <10}{2: <6}", "Method", "File Size", "Ratio");
        println!("------   ---------  ------");
        println!(
            "{0: <9}{1: <11}{2: <6}",
            &"Initial", &orig_string, &"100.0%"
        );
        upx(&path, orig_size, opt);
    } else {
        println!("Please install strip and/or upx to utilize this utility")
    }
}

pub fn path_exists(path: &Path) -> bool {
    fs::metadata(path).is_ok()
}
pub fn help() {
    println!("Binary shrinkage potential revealer\n");
    println!("USAGE: ");
    println!("\tsizeme [FLAGS] [OPTIONS] [<path>]");
    println!("FLAGS:");
    println!("\t-c, --compress      choose compression settings between 1-9 (9 is best)");
    println!("\t-h, -?, --help      prints help information");
    println!("\t-v, --version       prints version information");
}

pub fn check_upx(upx: &str) -> bool {
    match &upx {
        &"1" | &"2" | &"3" | &"4" | &"5" | &"6" | &"7" | &"8" | &"9" => return true,
        _ => return false,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("I need a path"),
        2 => match &args[1].trim() {
            &"-h" | &"-?" | &"help" => help(),
            &"-v" | &"--version" => println!("SizeMe Version: 0.3.0"),
            _ => {
                let path_string = get_path(&args[1]);
                let path = Path::new(&path_string);
                if path_exists(&path) {
                    if path.is_executable() {
                        let option = "1";
                        let orig_size = fs::metadata(&path).unwrap().len() as f64;
                        mk_files(path_string, orig_size, option);
                    } else {
                        println!("Sorry {} doesn't appear to be executable", &path_string);
                    }
                } else {
                    println!("Sorry {} doesn't appear to be a real path", &path_string);
                }
            }
        },
        4 => match &args[1].trim() {
            &"-h" | &"-?" | &"help" => help(),
            &"-v" | &"--version" => println!("SizeMe Version: 0.3.0"),
            &"-c" | &"--compress" => {
                if check_upx(&args[2]) == true {
                    let path_string = get_path(&args[3]);
                    let path = Path::new(&path_string);
                    if path_exists(&path) {
                        if path.is_executable() {
                            let orig_size = fs::metadata(&path).unwrap().len() as f64;
                            mk_files(path_string, orig_size, &args[2]);
                        } else {
                            println!("Sorry {} doesn't appear to be executable", path_string);
                        }
                    } else {
                        println!("Sorry {} doesn't appear to be a real path", path_string);
                    }
                } else {
                    println!("Compression must be a number 1-9");
                }
            }
            _ => {
                help();
            }
        },
        _ => help(),
    }
}
