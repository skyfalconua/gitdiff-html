//! `gitdiff-html` — turn a unified git diff into a single HTML page that looks
//! like a GitHub pull request.

mod assets;
mod cli;
mod diff;
mod render;

use std::fs;
use std::io::{self, Read, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
  let args: Vec<String> = std::env::args().skip(1).collect();
  let action = match cli::parse(args.into_iter()) {
    Ok(action) => action,
    Err(message) => {
      eprintln!("gitdiff-html: {}\n", message);
      eprint!("{}", cli::HELP);
      return ExitCode::from(2);
    }
  };

  let config = match action {
    cli::Action::Help => {
      print!("{}", cli::HELP);
      return ExitCode::SUCCESS;
    }
    cli::Action::Version => {
      println!("gitdiff-html {}", env!("CARGO_PKG_VERSION"));
      return ExitCode::SUCCESS;
    }
    cli::Action::Run(config) => config,
  };

  match run(&config) {
    Ok(()) => ExitCode::SUCCESS,
    Err(message) => {
      eprintln!("gitdiff-html: {}", message);
      ExitCode::FAILURE
    }
  }
}

/// Reads the diff, renders it and writes the result.
fn run(config: &cli::Config) -> Result<(), String> {
  let input = read_input(config)?;
  let files = diff::parse(&input);
  let page = render::render(&config.title, &files);

  match &config.output {
    Some(path) => fs::write(path, page)
      .map_err(|error| format!("cannot write {}: {}", path.display(), error)),
    None => {
      let stdout = io::stdout();
      let mut handle = stdout.lock();
      handle
        .write_all(page.as_bytes())
        .and_then(|()| handle.flush())
        .map_err(|error| format!("cannot write to stdout: {}", error))
    }
  }
}

/// Reads the diff from the given file or from standard input.
fn read_input(config: &cli::Config) -> Result<String, String> {
  match &config.input {
    Some(path) => {
      let bytes = fs::read(path)
        .map_err(|error| format!("cannot read {}: {}", path.display(), error))?;
      Ok(String::from_utf8_lossy(&bytes).into_owned())
    }
    None => {
      let mut bytes = Vec::new();
      io::stdin()
        .read_to_end(&mut bytes)
        .map_err(|error| format!("cannot read standard input: {}", error))?;
      Ok(String::from_utf8_lossy(&bytes).into_owned())
    }
  }
}
