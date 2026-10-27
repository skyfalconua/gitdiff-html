//! Command line parsing.

use std::path::PathBuf;

/// Title used when `--title` is not given.
pub const DEFAULT_TITLE: &str = "Diff";

/// What the program should do.
#[derive(Debug, Clone)]
pub enum Action {
  /// Render a page.
  Run(Config),
  /// Print the help text.
  Help,
  /// Print the version.
  Version,
}

/// Settings collected from the command line.
#[derive(Debug, Clone)]
pub struct Config {
  /// Title shown on the generated page.
  pub title: String,
  /// Where to read the diff from, `None` means standard input.
  pub input: Option<PathBuf>,
  /// Where to write the page to, `None` means standard output.
  pub output: Option<PathBuf>,
}

/// The help text.
pub const HELP: &str = concat!(
  "gitdiff-html ",
  env!("CARGO_PKG_VERSION"),
  "\nRender a unified git diff as a single self-contained HTML page.\n\n",
  "USAGE:\n",
  "  gitdiff-html [OPTIONS] [FILE]\n\n",
  "ARGS:\n",
  "  <FILE>  Read the diff from FILE instead of standard input\n\n",
  "OPTIONS:\n",
  "  -t, --title <TITLE>  Title of the generated page [default: Diff]\n",
  "  -o, --output <FILE>  Write the page to FILE instead of standard output\n",
  "  -i, --input <FILE>   Read the diff from FILE instead of standard input\n",
  "  -h, --help       Print this help\n",
  "  -V, --version      Print version information\n\n",
  "EXAMPLES:\n",
  "  git diff | gitdiff-html -t \"Fix login redirect\" > review.html\n",
  "  gitdiff-html -t \"Fix login redirect\" -o review.html changes.diff\n",
);

/// Parses the command line arguments (without the program name).
pub fn parse<I: Iterator<Item = String>>(args: I) -> Result<Action, String> {
  let mut config = Config {
    title: DEFAULT_TITLE.to_string(),
    input: None,
    output: None,
  };
  let mut args = args.peekable();

  while let Some(arg) = args.next() {
    match arg.as_str() {
      "-h" | "--help" => return Ok(Action::Help),
      "-V" | "--version" => return Ok(Action::Version),
      "-t" | "--title" => {
        let value = args
          .next()
          .ok_or_else(|| format!("missing value for {}", arg))?;
        config.title = value;
      }
      "-o" | "--output" => {
        let value = args
          .next()
          .ok_or_else(|| format!("missing value for {}", arg))?;
        config.output = Some(PathBuf::from(value));
      }
      "-i" | "--input" => {
        let value = args
          .next()
          .ok_or_else(|| format!("missing value for {}", arg))?;
        config.input = Some(PathBuf::from(value));
      }
      other => {
        if let Some(value) = other.strip_prefix("--title=") {
          config.title = value.to_string();
        } else if let Some(value) = other.strip_prefix("--output=") {
          config.output = Some(PathBuf::from(value));
        } else if let Some(value) = other.strip_prefix("--input=") {
          config.input = Some(PathBuf::from(value));
        } else if other.starts_with('-') && other.len() > 1 {
          return Err(format!("unknown option: {}", other));
        } else if config.input.is_some() {
          return Err(format!("unexpected argument: {}", other));
        } else {
          config.input = Some(PathBuf::from(other));
        }
      }
    }
  }

  Ok(Action::Run(config))
}

#[cfg(test)]
mod tests {
  use super::*;

  fn run(args: &[&str]) -> Result<Config, String> {
    match parse(args.iter().map(|arg| arg.to_string()))? {
      Action::Run(config) => Ok(config),
      _ => panic!("expected a run action"),
    }
  }

  #[test]
  fn reads_the_title() {
    let config = run(&["--title", "My review"]).unwrap();
    assert_eq!(config.title, "My review");
    let config = run(&["-t", "Short"]).unwrap();
    assert_eq!(config.title, "Short");
    let config = run(&["--title=Inline"]).unwrap();
    assert_eq!(config.title, "Inline");
  }

  #[test]
  fn reads_the_input_and_output() {
    let config = run(&["changes.diff", "-o", "page.html"]).unwrap();
    assert_eq!(config.input, Some(PathBuf::from("changes.diff")));
    assert_eq!(config.output, Some(PathBuf::from("page.html")));
  }

  #[test]
  fn defaults_are_sane() {
    let config = run(&[]).unwrap();
    assert_eq!(config.title, DEFAULT_TITLE);
    assert!(config.input.is_none());
    assert!(config.output.is_none());
  }

  #[test]
  fn rejects_unknown_options() {
    assert!(run(&["--nope"]).is_err());
    assert!(run(&["--title"]).is_err());
    assert!(run(&["a.diff", "b.diff"]).is_err());
  }
}
