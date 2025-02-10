use std::cmp::{max, min};
use itertools::Itertools;
use crate::tree::{Node, Tree};
use crate::error::Result;
use tree_sitter_powershell::language as powershell_language;
use crate::parser::{Parser};
use crate::visitor::LanguageVisitor;
use crate::rule::Rule;

fn build_powershell_tree(source: &str) -> Result<Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&powershell_language()).unwrap();

    let tree_sitter = parser.parse(source, None).unwrap();
    Ok(Tree::new(
        source.as_bytes(),
        tree_sitter,
    ))
}

#[derive(Default)]
pub struct Powershell;

impl Parser for Powershell {
    fn parse(&mut self, src: &str) -> Result<Option<(u64, String)>> {
        let tree = build_powershell_tree(src)?;
        let mut detection_rule = (
            LanguageVisitor::new(|c| {
                matches!(c, "sub_expression" | "assignment_expression" |
                    "array_expression" | "hash_literal_expression" |
                    // statement
                    "class_statement" | "sequence_statement" |
                    "data_statement" | "try_statement" | "trap_statement" |
                    "function_statement" | "do_statement" | "while_statement" | "for_statement" |
                    "foreach_statement" | "switch_statement" | "if_statement"
                )
            }),
            IsPowershellCmd::new()
        );
        tree.apply(&mut detection_rule)?;

        let start = match (detection_rule.0.start, detection_rule.1.start){
            (None, None) => None,
            (None, Some(x)) | (Some(x), None) => Some(x),
            (Some(x), Some(y)) => Some(min(x, y))
        };

        let end = match (detection_rule.0.end, detection_rule.1.end){
            (None, None) => None,
            (None, Some(x)) | (Some(x), None) => Some(x),
            (Some(x), Some(y)) => Some(max(x, y))
        };

        Ok(

            if detection_rule.0.is_matched || detection_rule.1.is_command {
                Some((
                    start.unwrap_or(0) as u64,
                    String::from(&src[start.unwrap_or(0)..end.unwrap_or(src.len())]
                    )
                ))
            }
            else {
                None
            }
        )
    }
}

pub struct IsPowershellCmd {
    is_command: bool,
    start: Option<usize>,
    end: Option<usize>
}

impl IsPowershellCmd {
    pub fn new() -> Self {
        Self {
            is_command: false,
            start: None,
            end: None
        }
    }
}

impl<'a> Rule<'a> for IsPowershellCmd  {
    fn enter(&mut self, node: &Node<'a>) -> Result<bool>{

        match node.kind() {
            "command" => {
                // matching criteria on command name
                if let Some(command_name) = node.named_child("command_name") {
                    match command_name.text()?.to_lowercase().split("-").collect::<Vec<&str>>().iter().next_tuple() {
                        Some((&"add", _)) | Some((&"clear", _)) | Some((&"close", _)) |
                        Some((&"copy", _)) | Some((&"enter", _)) | Some((&"exit", _)) |
                        Some((&"find", _)) | Some((&"format", _)) | Some((&"get", _)) |
                        Some((&"hide", _)) | Some((&"join", _)) | Some((&"lock", _)) |
                        Some((&"move", _)) | Some((&"new", _)) | Some((&"open", _)) |
                        Some((&"optimize", _)) | Some((&"pop", _)) | Some((&"push", _)) |
                        Some((&"redo", _)) | Some((&"remove", _)) | Some((&"rename", _)) |
                        Some((&"reset", _)) | Some((&"resize", _)) | Some((&"search", _)) |
                        Some((&"select", _)) | Some((&"set", _)) | Some((&"show", _)) |
                        Some((&"skip", _)) | Some((&"split", _)) | Some((&"step", _)) |
                        Some((&"switch", _)) | Some((&"undo", _)) | Some((&"unlock", _)) |
                        Some((&"watch", _)) | Some((&"connect", _)) | Some((&"disconnect", _)) |
                        Some((&"read", _)) | Some((&"receive", _)) | Some((&"send", _)) |
                        Some((&"write", _)) | Some((&"where", _)) | Some((&"compress", _)) |
                        Some((&"convert", _)) | Some((&"convertrom", _)) | Some((&"convertto", _)) |
                        Some((&"dismount", _)) | Some((&"edit", _)) | Some((&"expand", _)) |
                        Some((&"export", _)) | Some((&"group", _)) | Some((&"import", _)) |
                        Some((&"initialize", _)) | Some((&"limit", _)) | Some((&"merge", _)) |
                        Some((&"mount", _)) | Some((&"out", _)) | Some((&"publish", _)) |
                        Some((&"restore", _)) | Some((&"save", _)) | Some((&"sync", _)) |
                        Some((&"unpublish", _)) | Some((&"update", _)) | Some((&"debug", _)) |
                        Some((&"measure", _)) | Some((&"ping", _)) | Some((&"repair", _)) |
                        Some((&"resolve", _)) | Some((&"test", _)) | Some((&"trace", _)) |
                        Some((&"approve", _)) | Some((&"assert", _)) | Some((&"build", _)) |
                        Some((&"complete", _)) | Some((&"confirm", _)) | Some((&"deny", _)) |
                        Some((&"deploy", _)) | Some((&"disable", _)) | Some((&"enable", _)) |
                        Some((&"install", _)) | Some((&"invoke", _)) | Some((&"register", _)) |
                        Some((&"request", _)) | Some((&"restart", _)) | Some((&"resume", _)) |
                        Some((&"start", _)) | Some((&"stop", _)) | Some((&"submit", _)) |
                        Some((&"suspend", _)) | Some((&"uninstall", _)) | Some((&"unregister", _)) |
                        Some((&"wait", _)) | Some((&"block", _)) | Some((&"grant", _)) |
                        Some((&"protect", _)) | Some((&"revoke", _)) | Some((&"unblock", _)) |
                        Some((&"unprotect", _)) => {
                            self.start = Some(min(self.start.unwrap_or(node.start_abs()), node.start_abs()));
                            self.end = Some(max(self.end.unwrap_or(node.end_abs()), node.end_abs()));
                            self.is_command = true;
                        },
                        _ => ()
                    }
                }
            },
            _  => ()
        }

        Ok(true)
    }

    fn leave(&mut self, _node: &Node<'a>) -> Result<()>{
        Ok(())
    }
}