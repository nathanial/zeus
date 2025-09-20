use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyntaxKind {
    Normal,
    Comment,
    String,
    Number,
    Keyword,
    SpecialForm,
    Function,
    Constant,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HighlightSpan {
    pub start: usize,
    pub end: usize,
    pub kind: SyntaxKind,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LineHighlight {
    pub spans: Vec<HighlightSpan>,
}

#[derive(Default)]
struct LineState {
    in_string: bool,
}

const SPECIAL_FORMS: &[&str] = &[
    "define",
    "defun",
    "if",
    "quote",
    "lambda",
    "let",
    "let*",
    "cond",
    "and",
    "or",
    "progn",
    "when",
    "unless",
    "case",
    "letrec",
    "begin",
    "do",
    "loop",
    "catch",
    "throw",
    "unwind-protect",
    "block",
    "return-from",
    "tagbody",
    "go",
];

const BUILTIN_FUNCTIONS: &[&str] = &[
    "+",
    "-",
    "*",
    "/",
    "=",
    "/=",
    "<",
    "<=",
    ">",
    ">=",
    "list",
    "car",
    "cdr",
    "cons",
    "append",
    "reverse",
    "length",
    "nth",
    "nthcdr",
    "mapcar",
    "filter",
    "remove",
    "member",
    "reduce",
    "apply",
    "funcall",
    "print",
    "println",
    "gensym",
    "get",
    "put",
    "symbol-plist",
    "vector",
    "make-vector",
    "vector-ref",
    "vector-set!",
    "vector-length",
    "make-hash-table",
    "hash-set!",
    "hash-ref",
    "hash-remove!",
    "hash-keys",
    "char=",
    "char<",
    "char>",
    "char->integer",
    "integer->char",
    "integerp",
    "floatp",
    "rationalp",
    "numberp",
    "characterp",
    "vectorp",
    "hash-table-p",
];

const CONSTANTS: &[&str] = &["t", "nil"];

pub struct SyntaxHighlighter {
    special_forms: HashSet<&'static str>,
    builtins: HashSet<&'static str>,
    constants: HashSet<&'static str>,
    cache: Vec<LineHighlight>,
    dirty: bool,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            special_forms: SPECIAL_FORMS.iter().copied().collect(),
            builtins: BUILTIN_FUNCTIONS.iter().copied().collect(),
            constants: CONSTANTS.iter().copied().collect(),
            cache: Vec::new(),
            dirty: true,
        }
    }

    pub fn invalidate(&mut self) {
        self.dirty = true;
    }

    pub fn ensure(&mut self, content: &str) {
        if self.dirty {
            self.recompute(content);
        }
    }

    pub fn line(&self, index: usize) -> Option<&LineHighlight> {
        self.cache.get(index)
    }

    pub fn reset(&mut self, content: &str) {
        self.recompute(content);
    }

    fn recompute(&mut self, content: &str) {
        self.cache.clear();
        let mut state = LineState::default();
        for line in content.split('\n') {
            let highlight = self.highlight_line(line, &mut state);
            self.cache.push(LineHighlight { spans: highlight });
        }
        self.dirty = false;
    }

    fn highlight_line(&self, line: &str, state: &mut LineState) -> Vec<HighlightSpan> {
        let mut spans = Vec::new();
        let mut idx = 0usize;

        while idx < line.len() {
            if state.in_string {
                let (end, closed) = consume_string(line, idx);
                spans.push(HighlightSpan {
                    start: idx,
                    end,
                    kind: SyntaxKind::String,
                });
                idx = end;
                if closed {
                    state.in_string = false;
                } else {
                    break;
                }
                continue;
            }

            let Some((ch, ch_len)) = peek_char(line, idx) else {
                break;
            };

            if ch == ';' {
                spans.push(HighlightSpan {
                    start: idx,
                    end: line.len(),
                    kind: SyntaxKind::Comment,
                });
                break;
            } else if ch == '"' {
                state.in_string = true;
                continue;
            } else if ch.is_whitespace() {
                let end = consume_while(line, idx, char::is_whitespace);
                spans.push(HighlightSpan {
                    start: idx,
                    end,
                    kind: SyntaxKind::Normal,
                });
                idx = end;
                continue;
            } else if is_delimiter(ch) {
                spans.push(HighlightSpan {
                    start: idx,
                    end: idx + ch_len,
                    kind: SyntaxKind::Normal,
                });
                idx += ch_len;
                continue;
            } else {
                let end = consume_while(line, idx, |c| {
                    !c.is_whitespace() && !is_delimiter(c) && c != '"' && c != ';'
                });
                let token = &line[idx..end];
                let kind = self.classify_token(token);
                spans.push(HighlightSpan {
                    start: idx,
                    end,
                    kind,
                });
                idx = end;
            }
        }

        spans
    }

    fn classify_token(&self, token: &str) -> SyntaxKind {
        if token.is_empty() {
            return SyntaxKind::Normal;
        }
        if token.starts_with(':') {
            return SyntaxKind::Keyword;
        }

        let lower = token.to_ascii_lowercase();
        if self.constants.contains(lower.as_str()) {
            return SyntaxKind::Constant;
        }
        if self.special_forms.contains(lower.as_str()) {
            return SyntaxKind::SpecialForm;
        }
        if self.builtins.contains(lower.as_str()) {
            return SyntaxKind::Function;
        }
        if is_number(token) {
            return SyntaxKind::Number;
        }

        SyntaxKind::Normal
    }
}

fn peek_char(line: &str, start: usize) -> Option<(char, usize)> {
    if start >= line.len() {
        return None;
    }
    line[start..].chars().next().map(|ch| (ch, ch.len_utf8()))
}

fn consume_while<F>(line: &str, start: usize, mut predicate: F) -> usize
where
    F: FnMut(char) -> bool,
{
    let mut end = start;
    while let Some((ch, len)) = peek_char(line, end) {
        if !predicate(ch) {
            break;
        }
        end += len;
    }
    end
}

fn consume_string(line: &str, start: usize) -> (usize, bool) {
    let mut escaped = false;
    let mut first = true;

    for (offset, ch) in line[start..].char_indices() {
        let idx = start + offset;
        let current_end = idx + ch.len_utf8();

        if first {
            first = false;
            continue;
        }

        if escaped {
            escaped = false;
            continue;
        }

        match ch {
            '\\' => {
                escaped = true;
            }
            '"' => {
                return (current_end, true);
            }
            _ => {}
        }
    }

    (line.len(), false)
}

fn is_delimiter(ch: char) -> bool {
    matches!(
        ch,
        '(' | ')' | '[' | ']' | '{' | '}' | '\'' | '`' | ',' | '#'
    )
}

fn is_number(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }

    if token.contains('/') {
        let parts: Vec<&str> = token.split('/').collect();
        if parts.len() == 2 {
            return is_integer(parts[0]) && is_integer(parts[1]);
        }
    }

    is_integer(token) || token.parse::<f64>().is_ok()
}

fn is_integer(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }
    let mut chars = token.chars();
    if let Some(first) = chars.next() {
        if first == '+' || first == '-' {
            return chars.next().map_or(false, |c| c.is_ascii_digit())
                && chars.all(|c| c.is_ascii_digit());
        }
        if !first.is_ascii_digit() {
            return false;
        }
    } else {
        return false;
    }
    chars.all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::{consume_string, is_delimiter, is_number, SyntaxHighlighter, SyntaxKind};

    #[test]
    fn test_is_number_variants() {
        assert!(is_number("42"));
        assert!(is_number("-17"));
        assert!(is_number("3.14"));
        assert!(is_number("-0.25"));
        assert!(is_number("3/4"));
        assert!(!is_number("3/"));
        assert!(!is_number(""));
        assert!(!is_number("foo"));
    }

    #[test]
    fn test_consume_string() {
        let (end, closed) = consume_string("\"hello\"", 0);
        assert_eq!(end, 7);
        assert!(closed);

        let (end_unclosed, closed_unclosed) = consume_string("\"unterminated", 0);
        assert_eq!(end_unclosed, 13);
        assert!(!closed_unclosed);
    }

    #[test]
    fn test_is_delimiter_set() {
        assert!(is_delimiter('('));
        assert!(is_delimiter(')'));
        assert!(is_delimiter('#'));
        assert!(!is_delimiter('a'));
    }

    #[test]
    fn test_highlight_line_categories() {
        let mut highlighter = SyntaxHighlighter::new();
        highlighter.ensure("(if (< x 10) (print :ok))");
        let line = highlighter.line(0).expect("line present");
        assert!(line
            .spans
            .iter()
            .any(|span| span.kind == SyntaxKind::SpecialForm));
        assert!(line
            .spans
            .iter()
            .any(|span| span.kind == SyntaxKind::Function));
        assert!(line
            .spans
            .iter()
            .any(|span| span.kind == SyntaxKind::Number));
        assert!(line
            .spans
            .iter()
            .any(|span| span.kind == SyntaxKind::Keyword));
    }

    #[test]
    fn test_multiline_strings_state() {
        let mut highlighter = SyntaxHighlighter::new();
        let content = "(print \"hello\nworld\")";
        highlighter.ensure(content);
        let first_line = highlighter.line(0).unwrap();
        let second_line = highlighter.line(1).unwrap();
        assert!(first_line
            .spans
            .iter()
            .any(|span| span.kind == SyntaxKind::String));
        assert!(second_line
            .spans
            .iter()
            .any(|span| span.kind == SyntaxKind::String));
    }
}
