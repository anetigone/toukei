use super::lex_status::LineCtx;

/// 把一行文本映射成“类别”
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineKind {
    Blank,
    Comment,      // 纯注释
    DocComment,   // 文档注释
    Code,         // 纯代码
    Mixed,        // 代码+注释
}

pub trait Classifier: Send + Sync {
    /// 核心函数：根据上下文判断行类别
    fn classify(&self, line: LineCtx) -> (LineKind, Option<(usize, usize)>);
}

pub struct DefaultClassifier;

impl DefaultClassifier {
    pub fn new() -> Self {
        DefaultClassifier {}
    }
}

pub struct PythonClassifier;

impl PythonClassifier {
    pub fn new() -> Self {
        PythonClassifier {}
    }
}

impl Classifier for DefaultClassifier {
    fn classify(&self, mut line: LineCtx) -> (LineKind, Option<(usize, usize)>) {
        let s = line.trimmed().to_string();
        if s.is_empty() {
            return (LineKind::Blank, None);
        }
        
        if line.ctx().in_block_comment {
            if let Some((_, end)) = line.lang().block_comment {
                if let Some(pos) = s.find(end) {
                    line.ctx().in_block_comment = false;
                    if pos + end.len() == s.len() {
                        return (LineKind::Comment, None);
                    } else {
                        return (LineKind::Mixed, Some((pos + end.len(), s.len())));
                    }
                } else {
                    return (LineKind::Comment, None);
                }
            }
        }

        if let Some(prefix) = line.lang().line_comment {
            if s.starts_with(prefix) {
                return (LineKind::Comment, None);
            }
        }

        if let Some((start, end)) = line.lang().block_comment {
            if let Some(pos) = s.find(start) {
                let after = &s[pos + start.len()..];
                if let Some(end_pos) = after.find(end) {
                    if end_pos + end.len() == after.len() && pos == 0 {
                        return (LineKind::Comment, None);
                    } else {
                        return (LineKind::Mixed, Some((pos + end.len(), s.len())));
                    }
                }
                else {
                    line.ctx().in_block_comment = true;
                    let before = &s[..pos];
                    return if before.trim().is_empty() {
                        (LineKind::Comment, None)
                    } else {
                        (LineKind::Mixed, Some((0, pos)))
                    };
                }
            }
        }
        (LineKind::Code, None)
    }
}

impl Classifier for PythonClassifier {
    fn classify(&self, mut line: LineCtx) -> (LineKind, Option<(usize, usize)>) {
        let s = line.trimmed().to_string();
        if s.is_empty() {
            return (LineKind::Blank, None);
        }

        // Handle docstrings (""" or ''')
        if line.ctx().in_string {
            // Check for docstring end
            if s.contains("\"\"\"") || s.contains("'''") {
                line.ctx().in_string = false;
                // If line contains only docstring end, treat as comment
                if s.trim() == "\"\"\"" || s.trim() == "'''" {
                    return (LineKind::Comment, None);
                } else {
                    // Extract code after docstring
                    let end_pos = s.find("\"\"\"").or_else(|| s.find("'''")).unwrap();
                    let after = &s[end_pos + 3..];
                    if !after.trim().is_empty() {
                        return (LineKind::Mixed, Some((end_pos + 3, s.len())));
                    } else {
                        return (LineKind::Comment, None);
                    }
                }
            } else {
                return (LineKind::Comment, None);
            }
        }

        // Check for docstring start
        if s.starts_with("\"\"\"") || s.starts_with("'''") {
            let doc_start = if s.starts_with("\"\"\"") { "\"\"\"" } else { "'''" };
            if s.len() > 3 && s[3..].trim().contains(doc_start) {
                // Single line docstring
                return (LineKind::Comment, None);
            } else {
                // Multi-line docstring starts
                line.ctx().in_string = true;
                let after = &s[3..];
                if !after.trim().is_empty() {
                    return (LineKind::Mixed, Some((3, s.len())));
                } else {
                    return (LineKind::Comment, None);
                }
            }
        }

        // Regular line comments
        if let Some(prefix) = line.lang().line_comment {
            if s.starts_with(prefix) {
                return (LineKind::Comment, None);
            }
        }

        // Inline comments
        if let Some(comment_pos) = s.find('#') {
            let before = &s[..comment_pos];
            if !before.trim().is_empty() {
                return (LineKind::Mixed, Some((0, comment_pos)));
            } else {
                return (LineKind::Comment, None);
            }
        }

        (LineKind::Code, None)
    }
}
