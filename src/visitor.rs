use crate::error::Result;
use crate::rule::Rule;
use crate::tree::Node;
use std::cmp::{max, min};

pub struct LanguageVisitor {
    pub is_matched: bool,
    stack: Vec<bool>,
    pub start: Option<usize>,
    pub end: Option<usize>,
    match_fn: fn(&str) -> bool,
}

impl LanguageVisitor {
    pub fn new(match_fn: fn(&str) -> bool) -> Self {
        Self {
            is_matched: false,
            stack: vec![true],
            start: None,
            end: None,
            match_fn,
        }
    }
}

impl<'a> Rule<'a> for LanguageVisitor {
    fn enter(&mut self, node: &Node<'a>) -> Result<bool> {
        if node.child_count() > 1 {
            if (self.match_fn)(&node.kind()) {
                self.stack.push(true);
            }
        }
        Ok(true)
    }

    fn leave(&mut self, node: &Node<'a>) -> Result<()> {
        // invalidate the stack
        if node.kind() == "ERROR" || node.text()? == "" {
            for c in self.stack.iter_mut() {
                *c = false;
            }
        }

        if node.child_count() > 1 {
            if (self.match_fn)(&node.kind()) {
                if self.stack.pop().unwrap_or(false) {
                    self.start = Some(min(
                        self.start.unwrap_or(node.start_abs()),
                        node.start_abs(),
                    ));
                    self.end = Some(max(self.end.unwrap_or(node.end_abs()), node.end_abs()));
                    self.is_matched = true;
                }
            }
        }

        // empty node is a MISSING node => parsing error
        if node.text()? == "" {
            for c in self.stack.iter_mut() {
                *c = false;
            }
        }

        if self.is_matched && self.stack.last() == Some(&true) {
            self.start = Some(min(
                self.start.unwrap_or(node.start_abs()),
                node.start_abs(),
            ));
            self.end = Some(max(self.end.unwrap_or(node.end_abs()), node.end_abs()));
        }

        Ok(())
    }
}
