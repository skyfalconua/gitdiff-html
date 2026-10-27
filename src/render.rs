//! Renders a parsed diff into a single self-contained HTML page.
//!
//! The layout follows the GitHub pull request page: a sticky header with the
//! diff statistics and the view controls, a file list on the left and the diff
//! itself on the right.

use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use chrono::Local;

use crate::assets::{BOOT_JS, CSS, ICON_CHECK, ICON_CHEVRON, ICON_COPY, ICON_FILES, ICON_GIT, JS};
use crate::diff::{split_rows, DiffLine, FileDiff, LineKind};

/// Files with more changed lines than this start out collapsed.
pub const LARGE_DIFF_LINES: usize = 2000;

/// Aggregated numbers shown in the page header.
#[derive(Debug, Clone, Copy)]
struct Stats {
  files: usize,
  additions: usize,
  deletions: usize,
}

impl Stats {
  fn of(files: &[FileDiff]) -> Stats {
    let mut stats = Stats {
      files: files.len(),
      additions: 0,
      deletions: 0,
    };
    for file in files {
      stats.additions += file.additions;
      stats.deletions += file.deletions;
    }
    stats
  }
}

/// Renders the whole page.
pub fn render(title: &str, files: &[FileDiff]) -> String {
  let stats = Stats::of(files);
  let page = html! {
    (DOCTYPE)
    html lang="en" {
      head {
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1";
        meta name="generator" content="gitdiff-html";
        title { (title) }
        style { (PreEscaped(CSS)) }
        script { (PreEscaped(BOOT_JS)) }
      }
      body {
        (topbar(title, &stats))
        div.layout {
          (sidebar(files))
          main.content {
            @if files.is_empty() {
              p.file-note { "No changes to display." }
            } @else {
              @for (index, file) in files.iter().enumerate() {
                (article(file, index))
              }
            }
          }
        }
        script { (PreEscaped(JS)) }
      }
    }
  };
  page.render().into_string()
}

fn generated_now() -> String {
  Local::now().format("%Y-%m-%d %H:%M").to_string()
}

/// The sticky header with the title, the statistics and the view controls.
fn topbar(title: &str, stats: &Stats) -> Markup {
  let noun = if stats.files == 1 { "file" } else { "files" };
  html! {
    header.topbar {
      div.topbar-inner {
        div.brand {
          (PreEscaped(ICON_GIT))
          h1 { (title) }
          span.stats {
            (stats.files)
            " "
            (noun)
            " changed, "
            span.stat-add { "+" (stats.additions) }
            " "
            span.stat-del { "−" (stats.deletions) }
          }
          span.generated-time {
            "Generated "
            (generated_now())
          }
        }
        div.toolbar {
          div.segmented role="group" aria-label="Diff view" {
            button.seg-btn type="button" data-view-btn="unified" { "Unified" }
            button.seg-btn type="button" data-view-btn="split" { "Split" }
          }
          div.segmented role="group" aria-label="Theme" {
            button.seg-btn type="button" data-theme-btn="auto" { "Auto" }
            button.seg-btn type="button" data-theme-btn="light" { "Light" }
            button.seg-btn type="button" data-theme-btn="dark" { "Dark" }
          }
          button.btn type="button" data-action="expand-all" { "Expand all" }
          button.btn type="button" data-action="collapse-all" { "Collapse all" }
          button.btn.sidebar-toggle type="button" aria-label="Toggle file list" {
            (PreEscaped(ICON_FILES))
          }
        }
      }
    }
  }
}

/// The file list on the left hand side.
fn sidebar(files: &[FileDiff]) -> Markup {
  html! {
    aside.sidebar {
      div.sidebar-head {
        input.filter-input type="search" placeholder="Filter by path" aria-label="Filter files by path" autocomplete="off";
        span.filter-count { (files.len()) " files" }
      }
      ul.file-list {
        @for (index, file) in files.iter().enumerate() {
          (sidebar_item(file, index))
        }
      }
      p.no-matches.hidden { "No files match your filter." }
    }
  }
}

/// A single entry of the file list.
fn sidebar_item(file: &FileDiff, index: usize) -> Markup {
  let path = file.path();
  let anchor = format!("file-{}", index);
  let badge = format!("file-badge badge-{}", file.status.css_class());
  html! {
    li.file-item data-path=(path) {
      a.file-link href=(format!("#{}", anchor)) {
        span class=(badge) title=(file.status.label()) { (file.status.badge()) }
        span.file-path { (path) }
      }
    }
  }
}

/// One file of the diff, with its header and its diff body.
fn article(file: &FileDiff, index: usize) -> Markup {
  let path = file.path();
  let anchor = format!("file-{}", index);
  let badge = format!("file-badge badge-{}", file.status.css_class());
  let collapsed = file.binary || file.changed_lines() > LARGE_DIFF_LINES;
  let previous = match file.previous_path() {
    Some(old) => html! { span.file-old-path { "← " (old) } },
    None => html! {},
  };

  html! {
    article class=(file_class(collapsed)) id=(anchor) data-path=(path) {
      header.file-head {
        button.icon-btn.file-toggle type="button" aria-expanded=(if collapsed { "false" } else { "true" }) aria-label=(format!("Toggle {}", path)) {
          (PreEscaped(ICON_CHEVRON))
          span class=(badge) title=(file.status.label()) { (file.status.badge()) }
          span.file-name { (path) }
        }
        button.icon-btn.copy-btn type="button" data-copy=(path) data-tip="Copy path" aria-label=(format!("Copy path {}", path)) {
          (PreEscaped(ICON_COPY))
          (PreEscaped(ICON_CHECK))
        }
        (previous)
        span.file-stats {
          @if file.additions > 0 {
            span.stat-add { "+" (file.additions) }
          }
          @if file.deletions > 0 {
            span.stat-del { "−" (file.deletions) }
          }
        }
      }
      div.file-body {
        @if file.binary {
          p.file-note { "Binary file not shown." }
        } @else if file.hunks.is_empty() {
          p.file-note { "No content changes." }
        } @else {
          div.diff {
            table.diff.diff-unified {
              tbody { (unified_rows(file)) }
            }
            table.diff.diff-split {
              tbody { (split_rows_of(file)) }
            }
          }
        }
      }
    }
  }
}

/// The class attribute of a file block.
fn file_class(collapsed: bool) -> String {
  if collapsed {
    "file collapsed".to_string()
  } else {
    "file".to_string()
  }
}

/// The rows of the unified (single column) view.
fn unified_rows(file: &FileDiff) -> Markup {
  html! {
    @for hunk in &file.hunks {
      tr.hunk {
        td colspan="4" { (hunk.header) " " (hunk.heading) }
      }
      @for line in &hunk.lines {
        tr.diff-line {
          (number_cell(line.old_no, line.kind))
          (number_cell(line.new_no, line.kind))
          td class=(format!("marker {}", line.kind.css_class())) { (line.kind.marker()) }
          (code_cell(line))
        }
      }
    }
  }
}

/// The rows of the split (side by side) view.
fn split_rows_of(file: &FileDiff) -> Markup {
  html! {
    @for hunk in &file.hunks {
      tr.hunk {
        td colspan="6" { (hunk.header) " " (hunk.heading) }
      }
      @for row in split_rows(hunk) {
        tr.diff-line {
          (split_cell(row.left, true))
          (split_cell(row.right, false))
        }
      }
    }
  }
}

/// One half of a split view row.
fn split_cell(cell: Option<&DiffLine>, left: bool) -> Markup {
  match cell {
    None => html! {
      td.ln { }
      td.marker { }
      td.code { }
    },
    Some(line) => {
      let number = if left { line.old_no } else { line.new_no };
      html! {
        (number_cell(number, line.kind))
        td class=(format!("marker {}", line.kind.css_class())) { (line.kind.marker()) }
        (code_cell(line))
      }
    }
  }
}

/// A line number cell.
fn number_cell(number: Option<u32>, kind: LineKind) -> Markup {
  let text = number.map(|value| value.to_string()).unwrap_or_default();
  html! {
    td class=(format!("ln {}", kind.css_class())) { (text) }
  }
}

/// The cell holding the actual content of a line.
fn code_cell(line: &DiffLine) -> Markup {
  let marker = if line.no_newline {
    html! { span.no-newline { "\\ No newline at end of file" } }
  } else {
    html! {}
  };
  html! {
    td class=(format!("code {}", line.kind.css_class())) { (line.text) (marker) }
  }
}
