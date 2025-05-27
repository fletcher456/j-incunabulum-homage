// J Parse Tree Visualizer Module
// Provides visual representation of AST nodes for debugging

use crate::parser::JNode;
use crate::j_array::JArray;

pub struct ParseTreeVisualizer;

impl ParseTreeVisualizer {
    pub fn new() -> Self {
        ParseTreeVisualizer
    }

    // Generate a visual representation of the parse tree
    pub fn visualize(&self, node: &JNode) -> String {
        self.visualize_node(node, 0)
    }

    // Recursive function to visualize nodes with indentation
    fn visualize_node(&self, node: &JNode, depth: usize) -> String {
        let indent = "  ".repeat(depth);
        
        match node {
            JNode::Literal(array) => {
                format!("{}Literal: {}", indent, self.format_array(array))
            }
            
            JNode::MonadicVerb(verb, arg) => {
                format!("{}MonadicVerb: '{}'\n{}", 
                    indent, 
                    verb, 
                    self.visualize_node(arg, depth + 1)
                )
            }
            
            JNode::DyadicVerb(verb, left, right) => {
                format!("{}DyadicVerb: '{}'\n{}\n{}", 
                    indent, 
                    verb,
                    self.visualize_node(left, depth + 1),
                    self.visualize_node(right, depth + 1)
                )
            }
            
            JNode::AmbiguousVerb(verb, left, right) => {
                let mut result = format!("{}AmbiguousVerb: '{}'", indent, verb);
                
                if let Some(left_node) = left {
                    result.push_str(&format!("\n{}Left:\n{}", 
                        indent, 
                        self.visualize_node(left_node, depth + 1)
                    ));
                }
                
                if let Some(right_node) = right {
                    result.push_str(&format!("\n{}Right:\n{}", 
                        indent, 
                        self.visualize_node(right_node, depth + 1)
                    ));
                }
                
                result
            }
        }
    }

    // Format JArray for display
    fn format_array(&self, array: &JArray) -> String {
        if array.shape.rank() == 0 {
            format!("{}", array.data[0])
        } else {
            format!("[{}]", 
                array.data
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        }
    }
}