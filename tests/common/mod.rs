
#[macro_export]
macro_rules! target_path {
    ($path:tt) => {
        &format!("target/debug/{}", $path)
    };
}

#[macro_export]
macro_rules! assert_create_file {
    ($([$($command:tt),*] => $path:tt),*) => {
        $($(
            {
                use ::std::fs::{File, remove_file};
                use ::std::process::Command;

                let path = target_path!($path);
                let args = ::common::split_args($command);

                assert!(File::open(path).is_err());

                let cmd = &args[0];
                let mut cmd = Command::new(target_path!(cmd));
                for arg in args[0..].iter() {
                    cmd.arg(&arg);
                }
                cmd.output().expect(&format!("Failed execute command `{}`", $command));

                assert!(File::open(path).is_ok(),
                    format!("Command `{}` did not create file `{}`", $command, path)
                );

                remove_file(path).unwrap();
            }
        )*)*
    };
    ($($command:tt => $path:tt),*) => {

    };
}

#[macro_export]
macro_rules! create_file {
    ($path:tt, $content:tt) => {
        {
            use ::std::fs::File;
            use ::std::io::Write;

            let path = target_path!($path);
            let mut file = File::create(path)
                .expect(&format!("File: {} can not create.", path));
            file.write_all($content.as_bytes())
                .expect(&format!("File: {} wrong content write.", path));
        }
    };
}

pub fn split_args(line: &str) -> Vec<String> {
    let mut args = vec![];
    let mut start = 0;
    let mut quote_bch = 0;

    for (index, &bch) in line.as_bytes().iter().enumerate() {
        match bch {
            b' ' | b'\t' | b'\n' | b'\r' => {
                if quote_bch == 0 {
                    let arg = String::from_utf8_lossy(&line.as_bytes()[start..index]).to_string();
                    if !arg.is_empty() {
                        args.push(arg);
                    }
                    start = index + 1;
                }
            },
            b'"' | b'\'' => {
                if quote_bch == 0 {
                    quote_bch = bch;
                } else if quote_bch == bch {
                    quote_bch = 0;
                }
            },
            _ => ()
        }
    }
    args
}