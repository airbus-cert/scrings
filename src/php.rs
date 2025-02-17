use crate::error::Result;
use crate::parser::Parser;
use crate::tree::Tree;
use crate::visitor::LanguageVisitor;
use tree_sitter_php::LANGUAGE_PHP as php_language;

fn build_php_tree(source: &str) -> Result<Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&php_language.into())?;

    let tree_sitter = parser.parse(source, None).unwrap();
    Ok(Tree::new(source.as_bytes(), tree_sitter))
}

#[derive(Default)]
pub struct Php;

impl Parser for Php {
    fn parse(&mut self, src: &str) -> Result<Option<(u64, String)>> {
        let tree = build_php_tree(src)?;

        let mut detection_rule = LanguageVisitor::new(|c| {
            matches!(
                c,
                "if_statement"
                    | "switch_statement"
                    | "while_statement"
                    | "do_statement"
                    | "for_statement"
                    | "try_statement"
                    | "declare_statement"
                    | "echo_statement"
                    | "unset_statement"
                    | "const_declaration"
                    | "function_definition"
                    | "class_declaration"
                    | "interface_declaration"
                    | "trait_declaration"
                    | "enum_declaration"
                    | "namespace_definition"
                    | "namespace_use_declaration"
                    | "global_declaration"
                    | "function_static_declaration"
                    | "assignment_expression"
                    | "require_expression"
                    | "require_once_expression"
                    | "match_expression"
            )
        });

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
