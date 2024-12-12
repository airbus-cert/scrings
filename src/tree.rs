use tree_sitter::{Node as TreeNode, Tree as TreeSitter};
use crate::rule::{Rule};
use crate::error::{Result};
use tree_sitter_traversal2::{traverse, Order};

/// A node view use to explore the tree
/// without mutability
pub struct Node<'a> {
    /// The inner tree-sitter node
    node: TreeNode<'a>,
    /// Source reference
    source: &'a [u8]
}

/// Two nodes are equals if they have the same node id
impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.node.id() == other.node.id()
    }
}

impl<'a> Node<'a> {
    pub fn new(node: TreeNode<'a>, source: &'a [u8]) -> Self {
        Self {
            node,
            source
        }
    }

    pub fn child(&self, index: usize) -> Option<Node<'a>> {
        Some(Node::new(self.node.child(index)?, self.source))
    }

    pub fn named_child(&self, index: &str) -> Option<Node<'a>> {
        self.node.child_by_field_name(index).map(|node| Node::new(node, self.source))
    }

    pub fn iter(&self) -> NodeIterator<'a> {
        NodeIterator::new(
            Self::new(self.node, self.source),
            0,
            None,
            1
        )
    }

    pub fn range(&self, start: Option<usize>, end: Option<usize>, gap: Option<usize>) -> NodeIterator<'a> {
        NodeIterator::new(
            Self::new(self.node, self.source),
            start.unwrap_or(0),
            end,
            gap.unwrap_or(1)
        )
    }

    pub fn kind(&self) -> &'static str {
        self.node.kind()
    }

    pub fn start_rel(&self) -> usize {
        self.node.start_byte() - self.node.parent().unwrap().start_byte()
    }

    pub fn end_rel(&self) -> usize {
        self.node.end_byte() - self.node.parent().unwrap().start_byte()
    }

    pub fn start_abs(&self) -> usize {
        self.node.start_byte()
    }

    pub fn end_abs(&self) -> usize {
        self.node.end_byte()
    }

    pub fn is_extra(&self) -> bool {
        self.node.is_extra()
    }

    pub fn child_count(&self) -> usize {
        self.node.child_count()
    }
    pub fn text(&self) -> Result<&str>{
        Ok(self.node.utf8_text(self.source)?)
    }

    pub fn parent(&self) -> Option<Node<'a>> {
        self.node.parent().map(|node| Self::new(node, self.source))
    }


    pub fn get_parent_of_types(&self, kinds: Vec<&str>) -> Option<Node<'a>> {
        let mut current = self.parent();
        loop {
            if let Some(current_node) = current {
                if kinds.contains(&current_node.kind()) {
                    return Some(current_node);
                }
                current = current_node.parent();
            }
            else {
                return None;
            }
        }
    }

    fn apply(&self, rule: &mut impl Rule<'a>) -> Result<()> {
        let mut is_visiting = true;
        // Stack use to call 'leave' method when all children are handled
        let mut stack:Vec<(TreeNode, usize, bool)> = vec![];

        for node in traverse(self.node.walk(), Order::Pre) {
            stack.push((node, node.child_count(), is_visiting));
            if is_visiting {
                is_visiting = is_visiting && rule.enter(&Node::new(node, self.source))?;
            }

            // clean stack
            loop {
                let head = stack.last();
                if head.is_none() {
                    break;
                }

                let head_element = head.unwrap();

                // Do i have handle all children
                // if not continue to work on children
                if head_element.1 != 0 {
                    break;
                }

                if is_visiting {
                    rule.leave(&Node::new(head_element.0, self.source))?;
                }

                is_visiting = head_element.2;
                stack.pop();

                // decrement number of children handled
                if let Some(l) = stack.last_mut() {
                    l.1 = l.1 - 1;
                }
            }
        }
        Ok(())
    }

}

pub struct NodeIterator<'a> {
    inner: Node<'a>,
    index: usize,
    end: Option<usize>,
    gap : usize
}

impl<'a> NodeIterator<'a> {
    fn new(node: Node<'a>, start: usize, end: Option<usize>, gap: usize) -> Self{
        Self {
            inner: node,
            index : start,
            end,
            gap
        }
    }
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(end) = self.end {
            if self.index >= end {
                return None;
            }
        }

        match self.inner.child(self.index) {
            Some(node) => {
                self.index += self.gap;
                Some(node)
            },
            None => None
        }
    }
}
pub struct Tree<'a> {
    tree_sitter: TreeSitter,
    source: &'a[u8]
}

impl<'a> Tree<'a> {
    pub fn new(source: &'a[u8], tree_sitter: TreeSitter) -> Self {
        Self {
            tree_sitter,
            source
        }
    }

    pub fn apply<'b>(&'b self, rule: &mut (impl Rule<'b> + Sized)) -> Result<()> {
        let node = Node::new(self.tree_sitter.root_node(), self.source);
        node.apply(rule)
    }

    pub fn root(&self) -> Result<Node> {
        Ok(Node::new(self.tree_sitter.root_node(), self.source))
    }
}
