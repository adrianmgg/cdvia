use std::{env::current_dir, fs::canonicalize, io::Write as _, path::PathBuf, process::ExitCode};

use argh::{FromArgValue, FromArgs};
use eyre::{eyre, Context, ContextCompat};

#[cfg(feature = "shell-quote")]
use shell_quote::QuoteRefExt;

#[derive(FromArgs, Debug)]
/// cd to the directory you're already in, but via a different symlink
struct Cli {
    /// where to route through.
    #[argh(positional)]
    via: PathBuf,
    /// output format
    #[argh(option)]
    format: TargetFormat,
}

// TODO maybe switch from argh to clap see if their enum impl adds itself to the --help
#[derive(Debug, PartialEq, Eq, FromArgValue)]
enum TargetFormat {
    String,
    #[cfg(feature = "shell-quote-bash")]
    Bash,
    #[cfg(feature = "shell-quote-fish")]
    Fish,
    #[cfg(feature = "shell-quote-sh")]
    Sh,
}

fn main() -> ExitCode {
    simple_eyre::install().unwrap();
    let args: Cli = argh::from_env();
    match main_inner(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err:?}");
            ExitCode::FAILURE
        }
    }
}

fn main_inner(args: Cli) -> eyre::Result<()> {
    let pwd = current_dir().wrap_err("Failed to get current directory")?;
    let pwd_canon = canonicalize(&pwd).wrap_err_with(|| eyre!("Failed to canonicalize {pwd:?}"))?;
    let via = args.via;
    let via_canon = canonicalize(&via).wrap_err_with(|| eyre!("Failed to canonicalize {via:?}"))?;
    let route = pwd_canon
        .strip_prefix(&via_canon)
        // am just replacing this error since "prefix not found" doesn't add any additional useful info
        .map_err(|_| eyre!("{via_canon:?} is not a prefix of {pwd_canon:?}"))?;
    let new_pwd = via.join(route);
    match args.format {
        TargetFormat::String => {
            let new_pwd = new_pwd.to_str().wrap_err_with(|| {
                eyre!("Unable to convert new path ({new_pwd:?}) to string to be printed.")
            })?;
            println!("{new_pwd}");
        }
        #[cfg(feature = "shell-quote-bash")]
        TargetFormat::Bash => {
            let quoted: String = new_pwd.quoted(shell_quote::Bash);
            println!("cd {quoted}");
        }
        #[cfg(feature = "shell-quote-fish")]
        TargetFormat::Fish => {
            let quoted: String = new_pwd.quoted(shell_quote::Fish);
            println!("cd {quoted}");
        }
        #[cfg(feature = "shell-quote-sh")]
        TargetFormat::Sh => {
            let quoted: Vec<u8> = new_pwd.quoted(shell_quote::Sh);
            let mut stdout = std::io::stdout();
            stdout.write_all(b"cd ")?;
            stdout.write_all(&quoted)?;
            stdout.write_all(b"\n")?;
        }
    }
    Ok(())
}
