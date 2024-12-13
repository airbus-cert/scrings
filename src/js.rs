use crate::parser::Parser;
use crate::tree::Tree;
use crate::visitor::LanguageVisitor;
use crate::error::Result;
use tree_sitter_javascript::LANGUAGE as javascript_language;

fn build_javascript_tree(source: &str) -> Result<Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&javascript_language.into())?;

    let tree_sitter = parser.parse(source, None).unwrap();
    Ok(Tree::new(
        source.as_bytes(),
        tree_sitter,
    ))
}

#[derive(Default)]
pub struct Javascript;

impl Parser for Javascript {
    fn parse(&mut self, src: &str) -> Result<Option<(u64, String)>> {
        let tree = build_javascript_tree(src)?;

        let mut detection_rule = LanguageVisitor::new(|c| {
                matches!(c,
                    "function_declaration" |
                    "export_statement" | "debugger_statement" |
                    "declaration" |
                    "if_statement" | "switch_statement" |
                    "for_statement" | "for_in_statement" |
                    "while_statement" | "do_statement" |
                    "try_statement" | "with_statement"
                )
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