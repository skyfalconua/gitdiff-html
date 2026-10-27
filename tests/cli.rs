//! End to end tests: they run the compiled binary the way a user would.

use std::io::Write;
use std::process::{Command, Stdio};

const BINARY: &str = env!("CARGO_BIN_EXE_gitdiff-html");

/// Runs the binary with the given arguments and returns (stdout, stderr, success).
fn run(args: &[&str], stdin: &str) -> (String, String, bool) {
  let mut child = Command::new(BINARY)
    .args(args)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("failed to start the binary");

  child
    .stdin
    .as_mut()
    .expect("stdin")
    .write_all(stdin.as_bytes())
    .expect("failed to write to stdin");

  let output = child.wait_with_output().expect("failed to wait");
  (
    String::from_utf8_lossy(&output.stdout).into_owned(),
    String::from_utf8_lossy(&output.stderr).into_owned(),
    output.status.success(),
  )
}

const SAMPLE: &str = "\
diff --git a/src/main.rs b/src/main.rs
index 83db48f..bf269f4 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,3 +1,4 @@ fn main() {
 fn main() {
-  println!(\"old\");
+  println!(\"new\");
 }
";

#[test]
fn renders_a_page_from_stdin() {
  let (stdout, _, ok) = run(&[], SAMPLE);
  assert!(ok);
  assert!(stdout.starts_with("<!DOCTYPE html>"));
  assert!(stdout.contains("<title>Diff</title>"));
  assert!(stdout.contains("src/main.rs"));
  assert!(stdout.contains("println!"));
  // The page must not reference anything outside of itself.
  assert!(!stdout.contains("http://"));
  assert!(!stdout.contains("https://"));
  assert!(!stdout.contains("<link"));
}

#[test]
fn sets_the_title() {
  let (stdout, _, ok) = run(&["--title", "Fix login redirect"], SAMPLE);
  assert!(ok);
  assert!(stdout.contains("<title>Fix login redirect</title>"));
  assert!(stdout.contains("<h1>Fix login redirect</h1>"));
  assert!(stdout.contains("1 file changed,"));
  assert!(stdout.contains("<span class=\"stat-add\">+1</span>"));
  assert!(stdout.contains("<span class=\"stat-del\">−1</span>"));
}

#[test]
fn writes_to_a_file() {
  let path = std::env::temp_dir().join("gitdiff-html-test-output.html");
  let (stdout, _, ok) = run(&["-o", path.to_str().unwrap(), "-t", "From a file"], SAMPLE);
  assert!(ok);
  assert!(stdout.is_empty());
  let written = std::fs::read_to_string(&path).expect("output file");
  assert!(written.contains("<title>From a file</title>"));
  let _ = std::fs::remove_file(&path);
}

#[test]
fn reads_the_diff_from_a_file() {
  let path = std::env::temp_dir().join("gitdiff-html-test-input.diff");
  std::fs::write(&path, SAMPLE).expect("failed to write the diff");
  let (stdout, _, ok) = run(&[path.to_str().unwrap()], "");
  assert!(ok);
  assert!(stdout.contains("src/main.rs"));
  let _ = std::fs::remove_file(&path);
}

#[test]
fn shows_binary_files_as_a_notice() {
  let diff = "\
diff --git a/logo.png b/logo.png
index 4770f3f..a596e96 100644
Binary files a/logo.png and b/logo.png differ
";
  let (stdout, _, ok) = run(&[], diff);
  assert!(ok);
  assert!(stdout.contains("Binary file not shown"));
  // Binary files start out collapsed.
  assert!(stdout.contains("class=\"file collapsed\""));
}

#[test]
fn collapses_large_diffs() {
  let mut diff = String::from(
    "diff --git a/big.txt b/big.txt\nindex 83db48f..bf269f4 100644\n--- a/big.txt\n+++ b/big.txt\n@@ -1,2100 +1,2100 @@\n",
  );
  for index in 0..2100 {
    diff.push_str(&format!("+line {}\n", index));
  }
  let (stdout, _, ok) = run(&[], &diff);
  assert!(ok);
  assert!(stdout.contains("class=\"file collapsed\""));
}

#[test]
fn handles_an_empty_diff() {
  let (stdout, _, ok) = run(&[], "");
  assert!(ok);
  assert!(stdout.contains("No changes to display."));
}

#[test]
fn escapes_html_in_the_diff() {
  let diff = "\
diff --git a/x.html b/x.html
--- a/x.html
+++ b/x.html
@@ -1 +1 @@
-<script>alert(\"x\")</script>
+<b>bold &amp; broken</b>
";
  let (stdout, _, ok) = run(&[], diff);
  assert!(ok);
  assert!(stdout.contains("&lt;script&gt;"));
  assert!(!stdout.contains("<script>alert"));
}

#[test]
fn prints_help_and_version() {
  let (stdout, _, ok) = run(&["--help"], "");
  assert!(ok);
  assert!(stdout.contains("USAGE"));
  assert!(stdout.contains("--title"));

  let (stdout, _, ok) = run(&["--version"], "");
  assert!(ok);
  assert!(stdout.starts_with("gitdiff-html "));
}

#[test]
fn fails_on_unknown_options() {
  let (_, stderr, ok) = run(&["--nope"], "");
  assert!(!ok);
  assert!(stderr.contains("unknown option"));
}
