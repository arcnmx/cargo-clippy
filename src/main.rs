use std::path::Path;

#[cfg(not(test))]
fn main() {
    use std::{env, fs};
    use std::process::{self, Command};

    let path = env::current_exe().unwrap();
    let path = fs::canonicalize(path).unwrap();
    let path = path.parent().unwrap();
    let path = path.join("deps");

    let args = wrap_args(env::args(), path);
    let mut command = Command::new("cargo");
    command.args(&args);
    let mut child = command.spawn().unwrap_or_else(|e| panic!("{}", e));
    let exit_status = child.wait().unwrap_or_else(|e| panic!("{}", e));

    if let Some(code) = exit_status.code() {
        process::exit(code);
    }
}

fn wrap_args<T, I, P>(it: I, clippy_path: P) -> Vec<String>
    where T: AsRef<str>,
          I: IntoIterator<Item=T>,
          P: AsRef<Path> {

    let it = it.into_iter();
    let mut args = vec!["rustc".to_owned()];
    let mut has_double_hyphen = false;

    for arg in it.skip(2) {
        let arg = arg.as_ref().to_owned();
        has_double_hyphen |= &arg == "--";
        args.push(arg);
    }

    if !has_double_hyphen {
        args.push("--".to_owned());
    }
    args.push("-L".to_owned());
    args.push(clippy_path.as_ref().to_string_lossy().into_owned());
    args.push("-lclippy".to_owned());
    args.push("--test".to_owned());
    args.push("-Zextra-plugins=clippy".to_owned());
    args.push("-Zno-trans".to_owned());
    args
}

#[cfg(test)]
mod test {
    use super::wrap_args;

    #[test]
    fn test_wrap_args_no_double_hyphen() {
        let args = [
            "/usr/local/bin/cargo-clippy",
            "clippy",
            "--lib"
        ];
        let actual = wrap_args(&args, "/path/to/clippy");
        let expected = [
            "rustc",
            "--lib",
            "--",
            "-L",
            "/path/to/clippy",
            "-lclippy",
            "-Zextra-plugins=clippy",
            "-Zno-trans",
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_wrap_args_double_hyphen() {
        let args = [
            "/usr/local/bin/cargo-clippy",
            "clippy",
            "--lib",
            "--",
            "-Cprefer-dynamic"
        ];
        let actual = wrap_args(&args, "/path/to/clippy");
        let expected = [
            "rustc",
            "--lib",
            "--",
            "-Cprefer-dynamic",
            "-L",
            "/path/to/clippy",
            "-lclippy",
            "-Zextra-plugins=clippy",
            "-Zno-trans",
        ];
        assert_eq!(actual, expected);
    }
}
