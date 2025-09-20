# Text Editor Feature Plan

## Core Editing
- [x] Undo and redo stack (multi-level history with keyboard shortcuts).
- [x] Persistent file I/O (open, save, save-as for workspace files).
- [x] Configurable indentation (smart indent, spaces vs tabs, auto-indent on newline).
- [x] Multi-line text selection with copy, cut, and paste clipboard support.

## Navigation and Search
- [ ] Incremental find and replace with case sensitivity and regex options.
- [ ] Go-to-line and go-to-definition hooks (line jump plus interpreter integration).
- [ ] Scrollbar with position indicator and minimap or overview gutter.
- [ ] Line numbers and column ruler for orientation.

## Code Awareness
- [ ] Syntax highlighting driven by the Lisp tokenizer/theme palette.
- [ ] Structural editing helpers (balanced parentheses, sexp navigation, auto-format).
- [ ] Inline evaluation feedback (show result inline, error annotations, inspector hook).

## User Experience
- [ ] Configurable key bindings (support common editor shortcuts, user overrides).
- [ ] Theme switching and font preferences surfaced in settings.
- [ ] Status bar details (cursor position, selection length, file name, mode indicators).
- [ ] Autosave and crash recovery (dirty flag, periodic snapshots).
