#[macro_export]
macro_rules! target_path {
    ($path:tt) => {
        &format!("target/debug/{}", $path)
    };
}

#[macro_export]
macro_rules! delete_file {
    ($path:tt) => {
        ::std::fs::remove_file($path).expect(&format!("File: {} can not delete.", $path));
    };
}

#[macro_export]
macro_rules! create_file {
    ($path:tt, $content:tt) => {{
        use std::fs;
        use std::io::Write;
        use std::path::Path;

        let path = Path::new($path);
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).expect(&format!("Can't create dir: {:?}", dir));
        }
        let mut file = fs::File::create(path).expect(&format!("File: {:?} can not create.", path));
        file.write_all($content.as_bytes())
            .expect(&format!("File: {:?} wrong content write.", path));
    }};
}

#[macro_export]
macro_rules! run {
    ($($command:tt),*) => {
        $(
            {
                use ::std::process::Command;

                let args = ::common::split_args($command);

                let cmd = &args[0];
                let mut cmd = Command::new(target_path!(cmd));
                for arg in args[1..].iter() {
                    cmd.arg(&arg);
                }
                println!("command: {:?}", cmd);
                let output = cmd.output().expect(&format!("Failed execute command `{}`", $command));
                println!("{}", output.status);
                println!("{}", String::from_utf8_lossy(&output.stdout));
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        )*
    };
}

#[macro_export]
macro_rules! assert_create_file {
    ($([$($command:tt),*] => $path:tt),*) => {
        $($(
            assert_create_file!($command => $path);
        )*)*
    };
    ($($command:tt => $path:tt),*) => {
        $(
            {
                use ::std::fs::File;

                assert!(File::open($path).is_err(),
                    format!("File `{}` already exist", $path)
                );

                run!($command);

                assert!(File::open($path).is_ok(),
                    format!("Command `{}` did not create file `{}`", $command, $path)
                );

                delete_file!($path);
            }
        )*
    };
}

#[macro_export]
macro_rules! assert_output {
    ($([$($command:tt),*] => $out:tt),*) => {
        $($(
            assert_output!($command => $out);
        )*)*
    };
    ($($command:tt => $out:tt),*) => {
        $(
            {
                use ::std::process::Command;

                let args = ::common::split_args($command);
                let cmd = &args[0];
                let mut cmd = Command::new(target_path!(cmd));
                for arg in args[1..].iter() {
                    cmd.arg(&arg);
                }
                println!("command: {:?}", cmd);
                let output = cmd.output().expect(&format!("Failed execute command `{}`", $command));
                println!("{}", output.status);
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("{}", stdout);
                println!("{}", String::from_utf8_lossy(&output.stderr));

                let outs: Vec<_> = $out.trim().split("\n").collect();
                let stdouts: Vec<_> = stdout.trim().split("\n").collect();

                for out in outs.iter() {
                    assert!(stdouts.contains(out), "`{}` is not in {:?}", out, stdouts);
                }
            }
        )*
    };
}

pub fn split_args(line: &str) -> Vec<String> {
    let mut args = vec![];
    let mut start = 0;
    let mut quote_bch = 0;

    let push_arg = |args: &mut Vec<_>, slice| {
        let arg = String::from_utf8_lossy(slice).to_string().replace("\"", "");
        if !arg.is_empty() {
            args.push(arg);
        }
    };

    for (index, &bch) in line.as_bytes().iter().enumerate() {
        match bch {
            b' ' | b'\t' | b'\n' | b'\r' => {
                if quote_bch == 0 {
                    push_arg(&mut args, &line.as_bytes()[start..index]);
                    start = index + 1;
                }
            }
            b'"' | b'\'' => {
                if quote_bch == 0 {
                    quote_bch = bch;
                } else if quote_bch == bch {
                    quote_bch = 0;
                }
            }
            _ => (),
        }
    }
    push_arg(&mut args, &line.as_bytes()[start..]);
    args
}

#[macro_export]
macro_rules! assert_content {
    ($path:tt, $content:tt) => {{
        use std::fs;
        use std::io::Read;
        use std::path::Path;

        let path = Path::new($path);
        let mut file = fs::File::open(path).expect(&format!("File: {:?} can not open.", path));
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect(&format!("File: {:?} wrong content reading.", path));

        assert_eq!($content, contents);
    }};
}
