use crate::tree::Tree;
use tree_sitter_bash::language as bash_language;
use crate::parser::{Parser};
use crate::visitor::LanguageVisitor;
use crate::error::Result;

fn build_bash_tree(source: &str) -> Result<Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&bash_language())?;

    let tree_sitter = parser.parse(source, None).unwrap();
    Ok(Tree::new(
        source.as_bytes(),
        tree_sitter,
    ))
}

#[derive(Default)]
pub struct Bash;

impl Parser for Bash {
    fn parse(&mut self, src: &str) -> Result<Option<(u64, String)>> {
        let tree = build_bash_tree(src)?;

        let mut detection_rule = LanguageVisitor::new(|c| {
            matches!(c,
                "for_statement" | "if_statement" |
                "case_statement" | "unset_command" |
                "declaration_command" | "function_definition" |
                "compound_statement")
            }
        );

        tree.apply(&mut detection_rule)?;

        if detection_rule.is_matched {
            Ok(
                Some(
                    (
                        detection_rule.start.unwrap_or(0) as u64,
                        String::from(&src[detection_rule.start.unwrap_or(0)..detection_rule.end.unwrap_or(src.len())])
                    )
                )
            )
        }
        else {
            Ok(None)
        }
    }
}