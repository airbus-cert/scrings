use crate::error::Result;
use crate::parser::Parser;
use crate::tree::Tree;
use crate::visitor::LanguageVisitor;
use tree_sitter_sequel::LANGUAGE as sql_language;

fn build_sql_tree(source: &str) -> Result<Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&sql_language.into())?;

    let tree_sitter = parser.parse(source, None).unwrap();
    Ok(Tree::new(source.as_bytes(), tree_sitter))
}

#[derive(Default)]
pub struct Sql;

impl Parser for Sql {
    fn parse(&mut self, src: &str) -> Result<Option<(u64, String)>> {
        let tree = build_sql_tree(src)?;

        let mut detection_rule = LanguageVisitor::new(|c| matches!(c, "statement" | "transaction"));

        tree.apply(&mut detection_rule)?;

        if detection_rule.is_matched {
            Ok(Some((
                detection_rule.start.unwrap_or(0) as u64,
                String::from(
                    &src[detection_rule.start.unwrap_or(0)
                        ..detection_rule.end.unwrap_or(src.len())],
                ),
            )))
        } else {
            Ok(None)
        }
    }
}
