use std::{error::Error, fmt::Display};

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! run {
    ($cmd:expr, $($args:expr),*) => {{
        use std::process::{Command, Stdio};

        let args = vec![$($args,)*];
        if args.is_empty() {
            println!("Running `{}`", $cmd);
        } else {
            println!("Running `{} {}`", $cmd, &args.join(" "));
        }

        let output = Command::new($cmd)
            .args(&args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output();

        match output {
            Ok(output) if output.status.success() => Ok(()),
            Ok(output) => match output.status.code() {
                Some(code) => Err(BadExit::Code(code)),
                None => Err(BadExit::Signal),
            }
            Err(_) => {
                Err(BadExit::FailedToRun)
            }
        }
    }};
    ($cmd:expr, $($args:expr,)*) => {
        run!($cmd, $($args),*)
    };
}

pub enum BadExit {
    Code(i32),
    Signal,
    FailedToRun,
}

impl Error for BadExit {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for BadExit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadExit::Code(code) => f.write_fmt(format_args!("Exited with code {}", code)),
            BadExit::Signal => f.write_fmt(format_args!("Killed by signal")),
            BadExit::FailedToRun => f.write_fmt(format_args!("Failed to run")),
        }
    }
}

impl std::fmt::Debug for BadExit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}
