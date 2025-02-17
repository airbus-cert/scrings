use crate::error::Result;
use crate::parser::Parser;
use crate::rule::Rule;
use crate::tree::{Node, Tree};
use crate::visitor::LanguageVisitor;
use std::cmp::{max, min};
use tree_sitter_python::language as python_language;

fn build_python_tree(source: &str) -> crate::error::Result<Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&python_language()).unwrap();

    let tree_sitter = parser.parse(source, None).unwrap();
    Ok(Tree::new(source.as_bytes(), tree_sitter))
}

#[derive(Default)]
pub struct Python;

impl Parser for Python {
    fn parse(&mut self, src: &str) -> crate::error::Result<Option<(u64, String)>> {
        let tree = build_python_tree(src)?;

        let mut detection_rule = (
            LanguageVisitor::new(|c| {
                matches!(
                    c,
                    "if_statement"
                        | "for_statement"
                        | "while_statement"
                        | "try_statement"
                        | "with_statement"
                        | "function_definition"
                        | "class_definition"
                        | "decorated_definition"
                        | "match_statement"
                        | "future_import_statement"
                        | "import_from_statement"
                        | "assert_statement"
                        | "raise_statement"
                        | "pass_statement"
                        | "exec_statement"
                        | "import_statement"
                )
            }),
            IsPythonFunction::new(),
        );

        tree.apply(&mut detection_rule)?;

        let start = match (detection_rule.0.start, detection_rule.1.start) {
            (None, None) => None,
            (None, Some(x)) | (Some(x), None) => Some(x),
            (Some(x), Some(y)) => Some(min(x, y)),
        };

        let end = match (detection_rule.0.end, detection_rule.1.end) {
            (None, None) => None,
            (None, Some(x)) | (Some(x), None) => Some(x),
            (Some(x), Some(y)) => Some(max(x, y)),
        };

        Ok(
            if detection_rule.0.is_matched || detection_rule.1.is_function {
                Some((
                    start.unwrap_or(0) as u64,
                    String::from(&src[start.unwrap_or(0)..end.unwrap_or(src.len())]),
                ))
            } else {
                None
            },
        )
    }
}

pub struct IsPythonFunction {
    is_function: bool,
    start: Option<usize>,
    end: Option<usize>,
}

impl IsPythonFunction {
    pub fn new() -> Self {
        Self {
            is_function: false,
            start: None,
            end: None,
        }
    }
}

impl<'a> Rule<'a> for IsPythonFunction {
    // Match python function if in list
    fn enter(&mut self, node: &Node<'a>) -> Result<bool> {
        match node.kind() {
            "call" => {
                if let Some(function) = node.named_child("function") {
                    if matches!(
                        function.text().unwrap_or(""),
                        "requests.get"
                            | "requests.post"
                            | "os.system"
                            | "base64.b64decode"
                            | "b64decode"
                    ) {
                        self.is_function = true;
                        self.start = Some(min(
                            self.start.unwrap_or(node.start_abs()),
                            node.start_abs(),
                        ));
                        self.end = Some(max(self.end.unwrap_or(node.end_abs()), node.end_abs()));
                    }
                }
            }
            _ => (),
        }

        Ok(true)
    }

    fn leave(&mut self, _node: &Node<'a>) -> Result<()> {
        Ok(())
    }
}
