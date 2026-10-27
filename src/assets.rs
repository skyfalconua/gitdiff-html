//! Static assets (CSS, JavaScript, SVG icons) embedded into the generated page.
//!
//! The output must be a single self-contained file, so everything lives here as
//! plain strings and is injected into the template verbatim.

/// Icons used in the page, inlined so that no external resource is needed.
pub const ICON_CHEVRON: &str = r##"<svg class="icon" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true"><path fill="currentColor" d="M8 11.5 3.5 7l.9-.9L8 9.7l3.6-3.6.9.9z"/></svg>"##;

/// A "copy" icon, shown on the copy path buttons.
pub const ICON_COPY: &str = r##"<svg class="icon icon-copy" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true"><path fill="currentColor" d="M0 6.75C0 5.784.784 5 1.75 5h1.5a.75.75 0 0 1 0 1.5h-1.5a.25.25 0 0 0-.25.25v7.5c0 .138.112.25.25.25h7.5a.25.25 0 0 0 .25-.25v-1.5a.75.75 0 0 1 1.5 0v1.5A1.75 1.75 0 0 1 9.25 16h-7.5A1.75 1.75 0 0 1 0 14.25Zm5-5C5 .784 5.784 0 6.75 0h7.5C15.216 0 16 .784 16 1.75v9.5A1.75 1.75 0 0 1 14.25 13h-7.5A1.75 1.75 0 0 1 5 11.25Zm1.75-.25a.25.25 0 0 0-.25.25v9.5c0 .138.112.25.25.25h7.5a.25.25 0 0 0 .25-.25v-9.5a.25.25 0 0 0-.25-.25Z"/></svg>"##;

/// A check mark, shown after a successful copy.
pub const ICON_CHECK: &str = r##"<svg class="icon icon-check" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true"><path fill="currentColor" d="M13.78 4.22a.75.75 0 0 1 0 1.06l-7.25 7.25a.75.75 0 0 1-1.06 0L2.22 9.28a.75.75 0 1 1 1.06-1.06L6 10.94l6.72-6.72a.75.75 0 0 1 1.06 0Z"/></svg>"##;

/// The Git logo shown in the page header.
pub const ICON_GIT: &str = r##"<svg class="icon icon-brand" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true"><path fill="currentColor" d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.62-1.49 2.01-.23 4.12-1.01 4.12-4.55 0-.95-.34-1.73-.89-2.34.09-.23.39-1.11-.08-2.31 0 0-.73-.24-2.38.91a8.1 8.1 0 0 0-4.4 0c-1.65-1.15-2.38-.91-2.38-.91-.47 1.2-.17 2.08-.08 2.31-.55.61-.89 1.39-.89 2.34 0 3.53 2.1 4.32 4.11 4.55-.26.25-.5.73-.5 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.47.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8Z"/></svg>"##;

/// A "files" icon, used by the sidebar toggle on narrow screens.
pub const ICON_FILES: &str = r##"<svg class="icon" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true"><path fill="currentColor" d="M2 1.75C2 .784 2.784 0 3.75 0h3.5C8.216 0 9 .784 9 1.75v.5h3.25c.966 0 1.75.784 1.75 1.75v10.5A1.75 1.75 0 0 1 12.25 16h-8.5A1.75 1.75 0 0 1 2 14.25Zm1.75-.25a.25.25 0 0 0-.25.25v12.5c0 .138.112.25.25.25h8.5a.25.25 0 0 0 .25-.25v-10.5a.25.25 0 0 0-.25-.25H9v1.5A1.75 1.75 0 0 1 7.25 6h-3.5A1.75 1.75 0 0 1 2 4.25Zm1.75.75h3.5a.25.25 0 0 0 .25-.25v-.5a.25.25 0 0 0-.25-.25h-3.5a.25.25 0 0 0-.25.25v.5c0 .138.112.25.25.25Z"/></svg>"##;

/// The stylesheet of the generated page.
pub const CSS: &str = r##"
*, *::before, *::after {
  box-sizing: border-box;
}

:root {
  color-scheme: light;
  --canvas: #ffffff;
  --canvas-subtle: #f6f8fa;
  --canvas-inset: #eff2f5;
  --fg: #1f2328;
  --fg-muted: #656d76;
  --fg-subtle: #8c959f;
  --border: #d1d9e0;
  --border-muted: #d8dee4;
  --accent: #0969da;
  --accent-fg: #ffffff;
  --accent-subtle: #ddf4ff;
  --add-fg: #1a7f37;
  --add-bg: #e6ffec;
  --add-bg-strong: #ccffd8;
  --add-border: #aceebb;
  --del-fg: #cf222e;
  --del-bg: #ffebe9;
  --del-bg-strong: #ffd7d5;
  --del-border: #ffcecb;
  --hunk-bg: #ddf4ff;
  --hunk-border: #b6e3ff;
  --hunk-fg: #0550ae;
  --shadow-sm: 0 1px 0 rgba(31, 35, 40, 0.04), 0 1px 3px rgba(31, 35, 40, 0.08);
  --shadow-md: 0 8px 24px rgba(31, 35, 40, 0.12);
  --radius: 6px;
  --font-ui:
    -apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans", Helvetica,
    Arial, sans-serif, Apple Color Emoji, "Segoe UI Emoji";
  --font-mono:
    ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, "Liberation Mono",
    monospace;
}

html[data-theme="dark"] {
  color-scheme: dark;
  --canvas: #0d1117;
  --canvas-subtle: #151b23;
  --canvas-inset: #010409;
  --fg: #e6edf3;
  --fg-muted: #7d8590;
  --fg-subtle: #6e7681;
  --border: #3d444d;
  --border-muted: #212830;
  --accent: #4493f8;
  --accent-fg: #ffffff;
  --accent-subtle: #0c2d6b;
  --add-fg: #3fb950;
  --add-bg: #033a16;
  --add-bg-strong: #0a5227;
  --add-border: #238636;
  --del-fg: #f85149;
  --del-bg: #67060c;
  --del-bg-strong: #8e1519;
  --del-border: #f85149;
  --hunk-bg: #121d2f;
  --hunk-border: #1f3d63;
  --hunk-fg: #79c0ff;
  --shadow-sm: 0 1px 0 rgba(1, 4, 9, 0.4), 0 1px 3px rgba(1, 4, 9, 0.5);
  --shadow-md: 0 8px 24px rgba(1, 4, 9, 0.7);
}

html, body {
  height: 100%;
}
body {
  margin: 0;
  background: var(--canvas);
  color: var(--fg);
  font-family: var(--font-ui);
  font-size: 14px;
  line-height: 1.5;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

button {
  font: inherit;
  color: inherit;
}
a {
  color: var(--accent);
  text-decoration: none;
}
a:hover {
  text-decoration: underline;
}
.icon {
  flex: none;
  display: block;
}

/* ---------- top bar ---------- */

.topbar {
  flex: none;
  background: var(--canvas);
  border-bottom: 1px solid var(--border);
  box-shadow: var(--shadow-sm);
  z-index: 20;
}
.topbar-inner {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 10px 16px;
  flex-wrap: wrap;
}
.brand {
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
}
.brand .icon-brand {
  color: var(--fg);
}
.brand h1 {
  font-size: 16px;
  font-weight: 600;
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 46vw;
}
.brand .stats {
  color: var(--fg-muted);
  font-size: 12px;
  white-space: nowrap;
}
.generated-time {
  color: var(--fg-muted);
  font-size: 12px;
  white-space: nowrap;
}
.toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-left: auto;
  flex-wrap: wrap;
}
.segmented {
  display: inline-flex;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  overflow: hidden;
  background: var(--canvas);
}
.seg-btn {
  border: 0;
  background: transparent;
  padding: 5px 10px;
  font-size: 12px;
  color: var(--fg-muted);
  cursor: pointer;
  border-right: 1px solid var(--border);
}
.seg-btn:last-child {
  border-right: 0;
}
.seg-btn:hover {
  background: var(--canvas-subtle);
}
.seg-btn.is-active {
  background: var(--accent);
  color: var(--accent-fg);
  font-weight: 600;
}
.btn {
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--canvas);
  padding: 5px 10px;
  font-size: 12px;
  cursor: pointer;
  color: var(--fg);
}
.btn:hover {
  background: var(--canvas-subtle);
}
.btn:active {
  background: var(--canvas-inset);
}
.icon-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  height: 26px;
  border: 1px solid transparent;
  border-radius: var(--radius);
  background: transparent;
  color: var(--fg-muted);
  cursor: pointer;
  position: relative;
}
.icon-btn:hover {
  background: var(--canvas-subtle);
  color: var(--fg);
}
.icon-btn:focus-visible {
  outline: 2px solid var(--accent);
  outline-offset: 1px;
}
.sidebar-toggle {
  display: none;
}

/* ---------- layout ---------- */

.layout {
  display: flex;
  flex: 1;
  min-height: 0;
}
.sidebar {
  flex: none;
  max-width: 300px;
  border-right: 1px solid var(--border);
  background: var(--canvas-subtle);
  display: flex;
  flex-direction: column;
  min-height: 0;
}
.sidebar-head {
  flex: none;
  padding: 10px 12px;
  border-bottom: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.filter-input {
  width: 100%;
  font: inherit;
  font-size: 13px;
  padding: 5px 8px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--canvas);
  color: var(--fg);
}
.filter-input:focus {
  outline: 2px solid var(--accent);
  outline-offset: -1px;
  border-color: var(--accent);
}
.sidebar-count {
  font-size: 11px;
  color: var(--fg-muted);
}
.file-list {
  list-style: none;
  margin: 0;
  padding: 6px;
  overflow-y: auto;
  flex: 1;
  min-height: 0;
}
.file-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 5px 6px;
  border-radius: var(--radius);
  font-size: 13px;
}
.file-item:hover {
  background: var(--canvas);
}
.file-item.hidden {
  display: none;
}
.file-link {
  display: flex;
  align-items: center;
  gap: 6px;
  min-width: 0;
  flex: 1;
  color: var(--fg);
}
.file-link:hover {
  text-decoration: none;
}
.file-link:hover .file-path {
  text-decoration: underline;
}
.file-path {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  direction: rtl;
  text-align: left;
}
.file-badge {
  flex: none;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 700;
  line-height: 1;
  border: 1px solid;
}
.badge-added {
  color: var(--add-fg);
  border-color: var(--add-border);
  background: var(--add-bg);
}
.badge-deleted {
  color: var(--del-fg);
  border-color: var(--del-border);
  background: var(--del-bg);
}
.badge-modified {
  color: var(--hunk-fg);
  border-color: var(--hunk-border);
  background: var(--accent-subtle);
}
.badge-renamed,
.badge-copied {
  color: var(--accent);
  border-color: var(--border);
  background: var(--canvas);
}
.file-stats {
  flex: none;
  font-size: 11px;
  font-family: var(--font-mono);
  display: flex;
  gap: 5px;
}
.stat-add {
  color: var(--add-fg);
}
.stat-del {
  color: var(--del-fg);
}
.no-matches {
  padding: 12px;
  color: var(--fg-muted);
  font-size: 13px;
}
.no-matches.hidden {
  display: none;
}

/* ---------- main ---------- */

.content {
  flex: 1;
  min-width: 0;
  overflow-y: auto;
  padding: 16px;
}
.file {
  border: 1px solid var(--border);
  border-radius: var(--radius);
  margin-bottom: 16px;
  background: var(--canvas);
  overflow: hidden;
}
.file.hidden {
  display: none;
}
.file-head {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 10px;
  background: var(--canvas-subtle);
}
.file-head .file-toggle .icon {
  transition: transform 0.12s ease;
}
.file.collapsed .file-toggle .icon {
  transform: rotate(-90deg);
}
.file-name {
  font-family: var(--font-mono);
  font-size: 13px;
  font-weight: 600;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.file-old-path {
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--fg-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.file-head .file-stats {
  margin-left: auto;
}
.file-body {
  border-top: 1px solid var(--border-muted);
}
.file.collapsed .file-body {
  display: none;
}
.file-note {
  margin: 0;
  padding: 14px 16px;
  color: var(--fg-muted);
  font-size: 13px;
}
.file-note code {
  font-family: var(--font-mono);
}

/* ---------- diff tables ---------- */

.diff {
  overflow-x: auto;
}
.diff table {
  width: 100%;
  border-collapse: collapse;
  font-family: var(--font-mono);
  font-size: 12px;
  line-height: 1.55;
}
.diff td {
  padding: 0 8px;
  vertical-align: top;
  white-space: pre-wrap;
  overflow-wrap: anywhere;
}
.diff td.ln {
  width: 1%;
  white-space: nowrap;
  text-align: right;
  color: var(--fg-subtle);
  background: var(--canvas-subtle);
  user-select: none;
  border-right: 1px solid var(--border-muted);
}
.diff td.marker {
  width: 1%;
  text-align: center;
  user-select: none;
  color: var(--fg-muted);
}
.diff tr.hunk td {
  background: var(--hunk-bg);
  color: var(--hunk-fg);
  border-bottom: 1px solid var(--hunk-border);
  padding: 3px 8px;
  font-weight: 600;
}
.diff td.code.context {
  background: var(--canvas);
}
.diff td.marker.addition {
  color: var(--add-fg);
  background: var(--add-bg-strong);
}
.diff td.marker.deletion {
  color: var(--del-fg);
  background: var(--del-bg-strong);
}
.diff td.code.addition {
  background: var(--add-bg);
  color: var(--fg);
}
.diff td.code.deletion {
  background: var(--del-bg);
  color: var(--fg);
}
.diff td.ln.addition {
  background: var(--add-bg);
}
.diff td.ln.deletion {
  background: var(--del-bg);
}
.diff-split td.code {
  width: 40%;
}
.no-newline {
  display: block;
  color: var(--fg-subtle);
  font-style: italic;
}
html[data-view="unified"] .diff-split {
  display: none;
}
html[data-view="split"] .diff-unified {
  display: none;
}

/* ---------- copy button ---------- */

.copy-btn.is-copied {
  color: var(--add-fg);
}
.copy-btn .icon-check {
  display: none;
}
.copy-btn.is-copied .icon-copy {
  display: none;
}
.copy-btn.is-copied .icon-check {
  display: block;
}

/* ---------- narrow screens ---------- */

@media (max-width: 900px) {
  .sidebar {
    position: fixed;
    top: 0;
    bottom: 0;
    left: 0;
    width: min(320px, 85vw);
    transform: translateX(-102%);
    transition: transform 0.18s ease;
    box-shadow: var(--shadow-md);
    z-index: 40;
  }
  body.sidebar-open .sidebar {
    transform: none;
  }
  .sidebar-toggle {
    display: inline-flex;
  }
  .brand h1 {
    max-width: 50vw;
  }
}
@media (max-width: 600px) {
  .toolbar {
    width: 100%;
    margin-left: 0;
  }
}
"##;

/// Script that restores the preferences before the first paint.
pub const BOOT_JS: &str = r##"
(function(){
  try{
    var t=localStorage.getItem("gitdiff-html.theme")||"auto";
    var v=localStorage.getItem("gitdiff-html.view")||"unified";
    var d=(t==="auto"&&window.matchMedia&&window.matchMedia("(prefers-color-scheme: dark)").matches)?"dark":t;
    var e=document.documentElement;
    e.setAttribute("data-theme",d);
    e.setAttribute("data-theme-choice",t);
    e.setAttribute("data-view",v);
  }catch(e){}
})();
"##;

/// The behaviour of the generated page.
pub const JS: &str = r##"
(function(){
  "use strict";
  var root=document.documentElement;
  var body=document.body;
  var KEY_THEME="gitdiff-html.theme";
  var KEY_VIEW="gitdiff-html.view";

  function store(key,value){try{window.localStorage.setItem(key,value);}catch(e){}}
  function load(key){try{return window.localStorage.getItem(key);}catch(e){return null;}}
  function all(selector,scope){return Array.prototype.slice.call((scope||document).querySelectorAll(selector));}

  /* ---------- theme ---------- */

  function prefersDark(){
    return !!(window.matchMedia&&window.matchMedia("(prefers-color-scheme: dark)").matches);
  }
  function applyTheme(choice){
    var resolved=choice==="auto"?(prefersDark()?"dark":"light"):choice;
    root.setAttribute("data-theme",resolved);
    root.setAttribute("data-theme-choice",choice);
    all("[data-theme-btn]").forEach(function(btn){
      var active=btn.getAttribute("data-theme-btn")===choice;
      btn.classList.toggle("is-active",active);
      btn.setAttribute("aria-pressed",active?"true":"false");
    });
    store(KEY_THEME,choice);
  }
  if(window.matchMedia){
    var mq=window.matchMedia("(prefers-color-scheme: dark)");
    var onChange=function(){
      if((load(KEY_THEME)||"auto")==="auto"){applyTheme("auto");}
    };
    if(mq.addEventListener){mq.addEventListener("change",onChange);}
    else if(mq.addListener){mq.addListener(onChange);}
  }
  all("[data-theme-btn]").forEach(function(btn){
    btn.addEventListener("click",function(){applyTheme(btn.getAttribute("data-theme-btn"));});
  });
  applyTheme(load(KEY_THEME)||"auto");

  /* ---------- unified / split ---------- */

  function applyView(view){
    root.setAttribute("data-view",view);
    all("[data-view-btn]").forEach(function(btn){
      var active=btn.getAttribute("data-view-btn")===view;
      btn.classList.toggle("is-active",active);
      btn.setAttribute("aria-pressed",active?"true":"false");
    });
    store(KEY_VIEW,view);
  }
  all("[data-view-btn]").forEach(function(btn){
    btn.addEventListener("click",function(){applyView(btn.getAttribute("data-view-btn"));});
  });
  applyView(load(KEY_VIEW)||"unified");

  /* ---------- copy path ---------- */

  function copyText(text){
    if(navigator.clipboard&&window.isSecureContext){
      return navigator.clipboard.writeText(text);
    }
    return new Promise(function(resolve,reject){
      var area=document.createElement("textarea");
      area.value=text;
      area.setAttribute("readonly","");
      area.style.position="fixed";
      area.style.top="0";
      area.style.left="0";
      area.style.opacity="0";
      document.body.appendChild(area);
      area.select();
      var ok=false;
      try{ok=document.execCommand("copy");}catch(e){ok=false;}
      document.body.removeChild(area);
      ok?resolve():reject(new Error("copy failed"));
    });
  }
  function flashCopied(btn){
    var original=btn.getAttribute("data-tip");
    btn.classList.add("is-copied");
    btn.setAttribute("data-tip","Copied");
    window.setTimeout(function(){
      btn.classList.remove("is-copied");
      btn.setAttribute("data-tip",original);
    },1200);
  }
  all(".copy-btn").forEach(function(btn){
    btn.addEventListener("click",function(event){
      event.preventDefault();
      event.stopPropagation();
      var path=btn.getAttribute("data-copy");
      if(!path){return;}
      copyText(path).then(function(){flashCopied(btn);}).catch(function(){flashCopied(btn);});
    });
  });

  /* ---------- expand / collapse ---------- */

  function syncToggle(file){
    var toggle=file.querySelector(".file-toggle");
    if(!toggle){return;}
    var expanded=!file.classList.contains("collapsed");
    toggle.setAttribute("aria-expanded",expanded?"true":"false");
  }
  all(".file-toggle").forEach(function(toggle){
    toggle.addEventListener("click",function(){
      var file=toggle.closest(".file");
      if(!file){return;}
      file.classList.toggle("collapsed");
      syncToggle(file);
    });
  });
  all("[data-action]").forEach(function(btn){
    btn.addEventListener("click",function(){
      var action=btn.getAttribute("data-action");
      var collapse=action==="collapse-all";
      all(".file").forEach(function(file){
        file.classList.toggle("collapsed",collapse);
        syncToggle(file);
      });
    });
  });

  /* ---------- filter ---------- */

  var filter=document.querySelector(".filter-input");
  var counter=document.querySelector(".filter-count");
  var empty=document.querySelector(".no-matches");
  function applyFilter(){
    if(!filter){return;}
    var query=filter.value.trim().toLowerCase();
    var shown=0;
    all(".file-item").forEach(function(item){
      var path=(item.getAttribute("data-path")||"").toLowerCase();
      var match=query===""||path.indexOf(query)!==-1;
      item.classList.toggle("hidden",!match);
      if(match){shown++;}
    });
    all(".file").forEach(function(file){
      var path=(file.getAttribute("data-path")||"").toLowerCase();
      file.classList.toggle("hidden",query!==""&&path.indexOf(query)===-1);
    });
    if(counter){counter.textContent=query===""?shown+" files":shown+" of "+all(".file-item").length+" files";}
    if(empty){empty.classList.toggle("hidden",shown!==0);}
  }
  if(filter){filter.addEventListener("input",applyFilter);}
  applyFilter();

  /* ---------- sidebar (narrow screens) ---------- */

  all(".sidebar-toggle").forEach(function(btn){
    btn.addEventListener("click",function(){body.classList.toggle("sidebar-open");});
  });
  document.addEventListener("click",function(event){
    if(!body.classList.contains("sidebar-open")){return;}
    if(event.target.closest(".sidebar")||event.target.closest(".sidebar-toggle")){return;}
    body.classList.remove("sidebar-open");
  });
})();
"##;
