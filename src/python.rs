use crate::parser::Parser;
use crate::tree::Tree;
use crate::visitor::LanguageVisitor;
use tree_sitter_python::language as python_language;

fn build_python_tree(source: &str) -> crate::error::Result<Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&python_language()).unwrap();

    let tree_sitter = parser.parse(source, None).unwrap();
    Ok(Tree::new(
        source.as_bytes(),
        tree_sitter,
    ))
}

#[derive(Default)]
pub struct Python;

impl Parser for Python {
    fn parse(&mut self, src: &str) -> crate::error::Result<Option<String>> {
        let tree = build_python_tree(src)?;

        let mut detection_rule = LanguageVisitor::new(|c| {
            matches!(c,
                "if_statement" | "for_statement" |
                "while_statement" | "try_statement" |
                "with_statement" |
                "function_definition" | "class_definition" |
                "decorated_definition" | "match_statement" |
                "future_import_statement" | "import_from_statement" |
                "assert_statement" | "raise_statement" |
                "pass_statement" | "exec_statement"
            )
        }
        );

        tree.apply(&mut detection_rule)?;

        if detection_rule.is_matched {
            Ok(Some(String::from(&src[detection_rule.start.unwrap_or(0)..detection_rule.end.unwrap_or(src.len())])))
        }
        else {
            Ok(None)
        }
    }
}