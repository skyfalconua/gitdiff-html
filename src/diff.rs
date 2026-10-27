//! Parser for the unified diff format produced by `git diff`.
//!
//! The parser is intentionally forgiving: anything it does not recognise is
//! skipped instead of aborting, so a partially captured diff still renders.

/// How a file changed relative to its base version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Status {
  Added,
  #[default]
  Modified,
  Deleted,
  Renamed,
  Copied,
}

impl Status {
  /// Single letter badge, as used by GitHub.
  pub fn badge(self) -> &'static str {
    match self {
      Status::Added => "A",
      Status::Modified => "M",
      Status::Deleted => "D",
      Status::Renamed => "R",
      Status::Copied => "C",
    }
  }

  /// Human readable label.
  pub fn label(self) -> &'static str {
    match self {
      Status::Added => "Added",
      Status::Modified => "Modified",
      Status::Deleted => "Deleted",
      Status::Renamed => "Renamed",
      Status::Copied => "Copied",
    }
  }

  /// CSS modifier used for the badge in the file list.
  pub fn css_class(self) -> &'static str {
    match self {
      Status::Added => "added",
      Status::Modified => "modified",
      Status::Deleted => "deleted",
      Status::Renamed => "renamed",
      Status::Copied => "copied",
    }
  }
}

/// The kind of a single line inside a hunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineKind {
  Context,
  Added,
  Removed,
}

impl LineKind {
  /// The `+`/`-`/space marker git prints in front of the line.
  pub fn marker(self) -> &'static str {
    match self {
      LineKind::Context => " ",
      LineKind::Added => "+",
      LineKind::Removed => "-",
    }
  }

  /// CSS modifier for the row.
  pub fn css_class(self) -> &'static str {
    match self {
      LineKind::Context => "context",
      LineKind::Added => "addition",
      LineKind::Removed => "deletion",
    }
  }
}

/// A single line of a hunk.
#[derive(Debug, Clone)]
pub struct DiffLine {
  pub kind: LineKind,
  /// Line number on the left (old) side, if the line exists there.
  pub old_no: Option<u32>,
  /// Line number on the right (new) side, if the line exists there.
  pub new_no: Option<u32>,
  /// The line contents, without the leading marker.
  pub text: String,
  /// Whether git flagged this line with "No newline at end of file".
  pub no_newline: bool,
}

/// A contiguous block of changes (`@@ ... @@`).
#[derive(Debug, Clone)]
pub struct Hunk {
  /// The `@@ -a,b +c,d @@` part, verbatim.
  pub header: String,
  /// The section heading git appends after the `@@` marker.
  pub heading: String,
  pub lines: Vec<DiffLine>,
}

/// A file entry in the diff.
#[derive(Debug, Clone, Default)]
pub struct FileDiff {
  /// Path on the left side, `None` for newly added files.
  pub old_path: Option<String>,
  /// Path on the right side, `None` for deleted files.
  pub new_path: Option<String>,
  pub status: Status,
  /// Whether the file is binary (and therefore not shown inline).
  pub binary: bool,
  pub old_mode: Option<String>,
  pub new_mode: Option<String>,
  /// Similarity percentage for renames and copies.
  pub similarity: Option<String>,
  pub hunks: Vec<Hunk>,
  pub additions: usize,
  pub deletions: usize,
}

impl FileDiff {
  /// The path shown in the header and used by the "copy path" button.
  pub fn path(&self) -> &str {
    let raw = self
      .new_path
      .as_deref()
      .or(self.old_path.as_deref())
      .unwrap_or("(unknown)");
    strip_prefix(raw)
  }

  /// The previous path, for renamed and copied files.
  pub fn previous_path(&self) -> Option<&str> {
    match (self.old_path.as_deref(), self.new_path.as_deref()) {
      (Some(old), Some(new)) if strip_prefix(old) != strip_prefix(new) => {
        Some(strip_prefix(old))
      }
      _ => None,
    }
  }

  /// Number of added plus removed lines.
  pub fn changed_lines(&self) -> usize {
    self.additions + self.deletions
  }
}

/// A row of the split ("side by side") view: one left cell and one right
/// cell. A `None` cell means there is no line on that side of the pair.
///
/// The cells borrow from the hunk they were built from.
#[derive(Debug, Clone, Copy)]
pub struct SplitRow<'a> {
  pub left: Option<&'a DiffLine>,
  pub right: Option<&'a DiffLine>,
}

/// Parses a unified diff into the list of changed files.
pub fn parse(input: &str) -> Vec<FileDiff> {
  let mut files: Vec<FileDiff> = Vec::new();
  let mut current: Option<FileDiff> = None;
  let mut old_no = 0u32;
  let mut new_no = 0u32;

  for raw in input.lines() {
    let line = raw.strip_suffix('\r').unwrap_or(raw);

    // A new file block starts here.
    if let Some(rest) = line.strip_prefix("diff --git ") {
      files.extend(current.take());
      let (old_path, new_path) = parse_git_header(rest);
      current = Some(FileDiff {
        old_path,
        new_path,
        ..Default::default()
      });
      continue;
    }

    // Combined diffs (`git diff --cc`) are not supported; skip their block.
    if line.starts_with("diff --cc") || line.starts_with("diff --combined") {
      files.extend(current.take());
      continue;
    }

    let Some(file) = current.as_mut() else {
      continue;
    };

    // File level metadata is only valid before the first hunk.
    if file.hunks.is_empty() && apply_metadata(file, line) {
      continue;
    }

    // A new hunk starts here.
    if let Some((header, heading, old_start, new_start)) = parse_hunk_header(line) {
      file.hunks.push(Hunk {
        header,
        heading,
        lines: Vec::new(),
      });
      old_no = old_start;
      new_no = new_start;
      continue;
    }

    // Inside a hunk every line is content.
    let Some(hunk) = file.hunks.last_mut() else {
      continue;
    };

    if line.starts_with('\\') {
      let flagged = line.starts_with("\\ No newline at end of file");
      if let Some(last) = hunk.lines.last_mut().filter(|_| flagged) {
        last.no_newline = true;
      }
      continue;
    }

    let (kind, text) = match line.chars().next() {
      Some('+') => (LineKind::Added, &line[1..]),
      Some('-') => (LineKind::Removed, &line[1..]),
      Some(' ') => (LineKind::Context, &line[1..]),
      // An empty line is a context line with empty content; some tools
      // strip the trailing space.
      None => (LineKind::Context, ""),
      _ => continue,
    };

    let old_number = (kind != LineKind::Added).then_some(old_no);
    let new_number = (kind != LineKind::Removed).then_some(new_no);
    if old_number.is_some() {
      old_no += 1;
    }
    if new_number.is_some() {
      new_no += 1;
    }
    match kind {
      LineKind::Added => file.additions += 1,
      LineKind::Removed => file.deletions += 1,
      LineKind::Context => {}
    }

    hunk.lines.push(DiffLine {
      kind,
      old_no: old_number,
      new_no: new_number,
      text: text.to_string(),
      no_newline: false,
    });
  }

  files.extend(current.take());
  files
}

/// Pairs up the lines of a hunk for the side by side view.
pub fn split_rows(hunk: &Hunk) -> Vec<SplitRow<'_>> {
  let is_context = |line: &DiffLine| line.kind == LineKind::Context;
  let mut rows = Vec::new();

  // Alternating runs of context lines and change blocks.
  for group in hunk.lines.chunk_by(|a, b| is_context(a) == is_context(b)) {
    if is_context(&group[0]) {
      rows.extend(group.iter().map(|line| SplitRow {
        left: Some(line),
        right: Some(line),
      }));
    } else {
      // Pair removals with additions; the shorter side is padded.
      let (removed, added): (Vec<&DiffLine>, Vec<&DiffLine>) =
        group.iter().partition(|line| line.kind == LineKind::Removed);
      for offset in 0..removed.len().max(added.len()) {
        rows.push(SplitRow {
          left: removed.get(offset).copied(),
          right: added.get(offset).copied(),
        });
      }
    }
  }

  rows
}

/// Applies a file level metadata line (`new file mode`, `rename from`, `---`,
/// ...) to `file`. Returns `false` if the line is not metadata.
fn apply_metadata(file: &mut FileDiff, line: &str) -> bool {
  let value = |prefix: &str| line.strip_prefix(prefix).map(|rest| rest.trim().to_string());

  if let Some(mode) = value("new file mode ") {
    file.status = Status::Added;
    file.new_mode = Some(mode);
  } else if let Some(mode) = value("deleted file mode ") {
    file.status = Status::Deleted;
    file.old_mode = Some(mode);
  } else if let Some(mode) = value("old mode ") {
    file.old_mode = Some(mode);
  } else if let Some(mode) = value("new mode ") {
    file.new_mode = Some(mode);
  } else if let Some(path) = value("rename from ") {
    file.status = Status::Renamed;
    file.old_path = Some(unquote(&path));
  } else if let Some(path) = value("rename to ") {
    file.status = Status::Renamed;
    file.new_path = Some(unquote(&path));
  } else if let Some(path) = value("copy from ") {
    file.status = Status::Copied;
    file.old_path = Some(unquote(&path));
  } else if let Some(path) = value("copy to ") {
    file.status = Status::Copied;
    file.new_path = Some(unquote(&path));
  } else if let Some(index) = value("similarity index ").or_else(|| value("dissimilarity index "))
  {
    file.similarity = Some(index);
  } else if line.starts_with("index ") {
    // Blob hashes are not needed.
  } else if line.starts_with("Binary files ") || line.starts_with("GIT binary patch") {
    file.binary = true;
  } else if let Some(rest) = line.strip_prefix("--- ") {
    file.old_path = side_path(rest);
  } else if let Some(rest) = line.strip_prefix("+++ ") {
    file.new_path = side_path(rest);
  } else {
    return false;
  }
  true
}

/// Parses `@@ -1,4 +1,6 @@ heading` into its parts.
fn parse_hunk_header(line: &str) -> Option<(String, String, u32, u32)> {
  let rest = line.strip_prefix("@@")?.trim_start();
  let end = rest.find("@@")?;
  let ranges = rest[..end].trim();
  let heading = rest[end + 2..].trim().to_string();

  let mut tokens = ranges.split_whitespace();
  let old_start = parse_start(tokens.next()?.strip_prefix('-')?)?;
  let new_start = parse_start(tokens.next()?.strip_prefix('+')?)?;

  Some((format!("@@ {ranges} @@"), heading, old_start, new_start))
}

/// Parses the start of `12` or `12,7`.
fn parse_start(range: &str) -> Option<u32> {
  range.split(',').next()?.parse().ok()
}

/// Splits the `a/path b/path` pair of a `diff --git` header.
fn parse_git_header(rest: &str) -> (Option<String>, Option<String>) {
  let rest = rest.trim();

  // Quoted form: "a/x" "b/x"
  if let Some(quoted) = rest.strip_prefix('"') {
    if let Some(end) = find_quote_end(quoted) {
      let old = decode_escapes(&quoted[..end]);
      let remainder = quoted[end + 1..].trim_start();
      let new = (!remainder.is_empty()).then(|| unquote(remainder));
      return (Some(old), new);
    }
  }

  // Unquoted form: split at the first " b/" so paths containing spaces keep
  // working.
  if let Some((old, new)) = rest.split_once(" b/") {
    return (Some(old.to_string()), Some(format!("b/{new}")));
  }

  // Diffs without the a/ b/ prefixes.
  let mut tokens = rest.split_whitespace();
  (
    tokens.next().map(str::to_string),
    tokens.next().map(str::to_string),
  )
}

/// Finds the index of the closing quote of a quoted path.
fn find_quote_end(text: &str) -> Option<usize> {
  let mut chars = text.char_indices();
  while let Some((index, ch)) = chars.next() {
    match ch {
      '\\' => {
        chars.next()?;
      }
      '"' => return Some(index),
      _ => {}
    }
  }
  None
}

/// Extracts the path from the rest of a `--- a/path` / `+++ b/path` line.
///
/// Returns `None` for `/dev/null`, which means the file does not exist on
/// this side.
fn side_path(rest: &str) -> Option<String> {
  // Non-git diffs append a timestamp after a tab.
  let path = rest.split_once('\t').map_or(rest, |(path, _)| path).trim_end();
  (path != "/dev/null").then(|| unquote(path))
}

/// Removes the `a/` or `b/` prefix git adds to paths.
pub fn strip_prefix(path: &str) -> &str {
  path.strip_prefix("a/")
    .or_else(|| path.strip_prefix("b/"))
    .unwrap_or(path)
}

/// Removes the surrounding quotes git puts around unusual paths and decodes
/// the escapes inside. Unquoted input is returned as is.
fn unquote(path: &str) -> String {
  let path = path.trim();
  match path.strip_prefix('"') {
    Some(rest) => decode_escapes(rest.strip_suffix('"').unwrap_or(rest)),
    None => path.to_string(),
  }
}

/// Decodes the C-style escapes git uses inside quoted paths.
///
/// Non-ASCII bytes are written as three digit octal escapes (`\303\251`), so
/// the result is built as bytes and decoded as UTF-8 at the end.
fn decode_escapes(body: &str) -> String {
  let mut out: Vec<u8> = Vec::with_capacity(body.len());
  let mut bytes = body.bytes().peekable();

  while let Some(byte) = bytes.next() {
    if byte != b'\\' {
      out.push(byte);
      continue;
    }
    match bytes.next() {
      Some(b'n') => out.push(b'\n'),
      Some(b't') => out.push(b'\t'),
      Some(b'r') => out.push(b'\r'),
      Some(b'a') => out.push(0x07),
      Some(b'b') => out.push(0x08),
      Some(b'f') => out.push(0x0c),
      Some(b'v') => out.push(0x0b),
      Some(digit @ b'0'..=b'7') => {
        let mut value = u32::from(digit - b'0');
        for _ in 0..2 {
          match bytes.peek() {
            Some(&next @ b'0'..=b'7') => {
              value = value * 8 + u32::from(next - b'0');
              bytes.next();
            }
            _ => break,
          }
        }
        out.push((value & 0xff) as u8);
      }
      // `\"`, `\\` and any unknown escape stand for the character itself.
      Some(other) => out.push(other),
      None => {}
    }
  }

  String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "\
diff --git a/src/main.rs b/src/main.rs
index 83db48f..bf269f4 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,6 +1,7 @@ fn main() {
 fn main() {
-  println!(\"old\");
+  println!(\"new\");
+  println!(\"extra\");
 }
diff --git a/new.txt b/new.txt
new file mode 100644
index 0000000..3b18e51
--- /dev/null
+++ b/new.txt
@@ -0,0 +1 @@
+hello
diff --git a/img.png b/img.png
index a1b2c3d..e4f5a6b 100644
Binary files a/img.png and b/img.png differ
";

  #[test]
  fn parses_three_files() {
    let files = parse(SAMPLE);
    assert_eq!(files.len(), 3);
    assert_eq!(files[0].path(), "src/main.rs");
    assert_eq!(files[0].status, Status::Modified);
    assert_eq!(files[0].additions, 2);
    assert_eq!(files[0].deletions, 1);
    assert_eq!(files[0].hunks.len(), 1);
    assert_eq!(files[0].hunks[0].heading, "fn main() {");
    assert_eq!(files[1].status, Status::Added);
    assert_eq!(files[1].path(), "new.txt");
    assert!(files[2].binary);
    assert_eq!(files[2].status, Status::Modified);
  }

  #[test]
  fn tracks_line_numbers() {
    let files = parse(SAMPLE);
    let lines = &files[0].hunks[0].lines;
    assert_eq!(lines[0].kind, LineKind::Context);
    assert_eq!(lines[0].old_no, Some(1));
    assert_eq!(lines[0].new_no, Some(1));
    assert_eq!(lines[1].kind, LineKind::Removed);
    assert_eq!(lines[1].old_no, Some(2));
    assert_eq!(lines[1].new_no, None);
    assert_eq!(lines[2].kind, LineKind::Added);
    assert_eq!(lines[2].old_no, None);
    assert_eq!(lines[2].new_no, Some(2));
  }

  #[test]
  fn pairs_split_rows() {
    let files = parse(SAMPLE);
    let rows = split_rows(&files[0].hunks[0]);
    assert_eq!(rows.len(), 4);
    assert!(rows[1].left.is_some());
    assert!(rows[1].right.is_some());
    // The extra addition has no counterpart on the left.
    assert!(rows[2].left.is_none());
    assert!(rows[2].right.is_some());
  }

  #[test]
  fn handles_renames_and_modes() {
    let diff = "\
diff --git a/old.txt b/new.txt
similarity index 87%
rename from old.txt
rename to new.txt
old mode 100644
new mode 100755
";
    let files = parse(diff);
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].status, Status::Renamed);
    assert_eq!(files[0].path(), "new.txt");
    assert_eq!(files[0].previous_path(), Some("old.txt"));
    assert_eq!(files[0].new_mode.as_deref(), Some("100755"));
  }

  #[test]
  fn handles_no_newline_marker() {
    let diff = "\
diff --git a/a.txt b/a.txt
--- a/a.txt
+++ b/a.txt
@@ -1 +1 @@
-old
\\ No newline at end of file
+new
\\ No newline at end of file
";
    let files = parse(diff);
    let lines = &files[0].hunks[0].lines;
    assert!(lines[0].no_newline);
    assert!(lines[1].no_newline);
  }

  #[test]
  fn strips_prefixes_but_keeps_directories() {
    assert_eq!(strip_prefix("a/src/lib.rs"), "src/lib.rs");
    assert_eq!(strip_prefix("b/a/b.txt"), "a/b.txt");
    assert_eq!(strip_prefix("README.md"), "README.md");
  }

  #[test]
  fn splits_the_git_header_into_two_paths() {
    let (old, new) = parse_git_header("a/docs/old.txt b/docs/old.txt");
    assert_eq!(old.as_deref(), Some("a/docs/old.txt"));
    assert_eq!(new.as_deref(), Some("b/docs/old.txt"));

    let (old, new) = parse_git_header("a/with space.txt b/with space.txt");
    assert_eq!(old.as_deref(), Some("a/with space.txt"));
    assert_eq!(new.as_deref(), Some("b/with space.txt"));
  }

  #[test]
  fn dev_null_means_the_file_is_absent_on_that_side() {
    let diff = "\
diff --git a/docs/gone.txt b/docs/gone.txt
deleted file mode 100644
index acc4bcb..0000000
--- a/docs/gone.txt
+++ /dev/null
@@ -1 +0,0 @@
-old text
";
    let files = parse(diff);
    assert_eq!(files[0].path(), "docs/gone.txt");
    assert!(files[0].new_path.is_none());
    assert_eq!(files[0].previous_path(), None);

    let diff = "\
diff --git a/docs/fresh.txt b/docs/fresh.txt
new file mode 100644
index 0000000..d82766f
--- /dev/null
+++ b/docs/fresh.txt
@@ -0,0 +1 @@
+new file
";
    let files = parse(diff);
    assert_eq!(files[0].path(), "docs/fresh.txt");
    assert!(files[0].old_path.is_none());
  }

  #[test]
  fn decodes_quoted_paths_in_the_git_header() {
    let (old, new) = parse_git_header(r#""a/caf\303\251.txt" "b/caf\303\251.txt""#);
    assert_eq!(old.as_deref(), Some("a/café.txt"));
    assert_eq!(new.as_deref(), Some("b/café.txt"));
  }

  #[test]
  fn unquote_decodes_escapes() {
    assert_eq!(unquote(r#""tab\there""#), "tab\there");
    assert_eq!(unquote(r#""q\"uote""#), "q\"uote");
    assert_eq!(unquote(r#""back\\slash""#), "back\\slash");
    assert_eq!(unquote(r#""caf\303\251""#), "café");
    assert_eq!(unquote("plain"), "plain");
  }

  #[test]
  fn quoted_paths_survive_a_full_parse() {
    let diff = "\
diff --git \"a/caf\\303\\251.txt\" \"b/caf\\303\\251.txt\"
new file mode 100644
";
    let files = parse(diff);
    assert_eq!(files[0].path(), "café.txt");
  }

  #[test]
  fn multiple_change_blocks_in_one_hunk() {
    let diff = "\
diff --git a/a.txt b/a.txt
--- a/a.txt
+++ b/a.txt
@@ -1,5 +1,5 @@
-one
+uno
 two
-three
-four
+tres
 five
";
    let files = parse(diff);
    let rows = split_rows(&files[0].hunks[0]);
    // change(1:1), context, change(2:1 -> padded), context
    assert_eq!(rows.len(), 5);
    assert!(rows[3].left.is_some());
    assert!(rows[3].right.is_none());
  }
}
