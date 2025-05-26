#[cfg(test)]
mod tests {
    use crate::lalr_parser::LalrParser;
    use crate::tokenizer::JTokenizer;
    use crate::visualizer::ParseTreeVisualizer;
    use crate::parser::{JNode, ParseError};
    
    fn parse_and_visualize(expression: &str) -> (Result<JNode, ParseError>, String) {
        let tokenizer = JTokenizer::new();
        let parser = LalrParser::new();
        let visualizer = ParseTreeVisualizer::new();
        
        println!("Testing expression: {}", expression);
        
        // Tokenize
        let tokens = match tokenizer.tokenize(expression) {
            Ok(tokens) => {
                println!("Tokens: {:?}", tokens);
                tokens
            },
            Err(err) => {
                let error_msg = format!("Tokenizer error: {}", err);
                println!("{}", error_msg);
                return (Err(ParseError::InvalidExpression(error_msg)), error_msg);
            }
        };
        
        // Parse
        let ast_result = parser.parse(tokens);
        
        // Visualize
        let visualization = match &ast_result {
            Ok(ast) => visualizer.visualize(ast),
            Err(err) => format!("Parse failed: {}", err),
        };
        
        println!("Parse tree:\n{}", visualization);
        
        (ast_result, visualization)
    }
    
    #[test]
    fn test_simple_scalar() {
        let (result, _viz) = parse_and_visualize("5");
        assert!(result.is_ok());
        
        if let Ok(ast) = result {
            match ast {
                JNode::Literal(array) => {
                    assert_eq!(array.data[0], 5);
                    assert_eq!(array.rank, 0);
                },
                _ => panic!("Expected literal node for scalar"),
            }
        }
    }
    
    #[test]
    fn test_simple_monadic() {
        let (result, _viz) = parse_and_visualize("~3");
        assert!(result.is_ok());
        
        if let Ok(ast) = result {
            match ast {
                JNode::AmbiguousVerb(verb, left, right) => {
                    assert_eq!(verb, '~');
                    assert!(left.is_none()); // Monadic should have no left operand
                    assert!(right.is_some()); // Should have right operand
                    
                    if let Some(right_node) = right {
                        match *right_node {
                            JNode::Literal(ref array) => {
                                assert_eq!(array.data[0], 3);
                            },
                            _ => panic!("Expected literal argument"),
                        }
                    }
                },
                _ => panic!("Expected ambiguous verb node for monadic"),
            }
        }
    }
    
    #[test]
    fn test_simple_dyadic() {
        let (result, _viz) = parse_and_visualize("2+3");
        assert!(result.is_ok());
        
        if let Ok(ast) = result {
            match ast {
                JNode::AmbiguousVerb(verb, left, right) => {
                    assert_eq!(verb, '+');
                    assert!(left.is_some()); // Dyadic should have left operand
                    assert!(right.is_some()); // Should have right operand
                    
                    if let Some(left_node) = left {
                        match *left_node {
                            JNode::Literal(ref array) => assert_eq!(array.data[0], 2),
                            _ => panic!("Expected literal left operand"),
                        }
                    }
                    
                    if let Some(right_node) = right {
                        match *right_node {
                            JNode::Literal(ref array) => assert_eq!(array.data[0], 3),
                            _ => panic!("Expected literal right operand"),
                        }
                    }
                },
                _ => panic!("Expected ambiguous verb node for dyadic"),
            }
        }
    }
    
    #[test]
    fn test_critical_precedence_case() {
        // This is our critical test case
        let (result, viz) = parse_and_visualize("~3+~3");
        
        println!("CRITICAL TEST - ~3+~3 parsing:");
        println!("{}", viz);
        
        assert!(result.is_ok());
        
        if let Ok(ast) = result {
            match ast {
                JNode::AmbiguousVerb(verb, left, right) => {
                    assert_eq!(verb, '+');
                    assert!(left.is_some());
                    assert!(right.is_some());
                    
                    // Verify left side is AmbiguousVerb(~, None, Some(3))
                    if let Some(left_node) = left {
                        match *left_node {
                            JNode::AmbiguousVerb(left_verb, ref left_left, ref left_right) => {
                                assert_eq!(left_verb, '~');
                                assert!(left_left.is_none()); // Monadic
                                assert!(left_right.is_some());
                                
                                if let Some(ref left_arg) = *left_right {
                                    match **left_arg {
                                        JNode::Literal(ref array) => assert_eq!(array.data[0], 3),
                                        _ => panic!("Expected literal in left monadic"),
                                    }
                                }
                            },
                            _ => panic!("Expected monadic verb on left side"),
                        }
                    }
                    
                    // Verify right side is AmbiguousVerb(~, None, Some(3))
                    if let Some(right_node) = right {
                        match *right_node {
                            JNode::AmbiguousVerb(right_verb, ref right_left, ref right_right) => {
                                assert_eq!(right_verb, '~');
                                assert!(right_left.is_none()); // Monadic
                                assert!(right_right.is_some());
                                
                                if let Some(ref right_arg) = *right_right {
                                    match **right_arg {
                                        JNode::Literal(ref array) => assert_eq!(array.data[0], 3),
                                        _ => panic!("Expected literal in right monadic"),
                                    }
                                }
                            },
                            _ => panic!("Expected monadic verb on right side"),
                        }
                    }
                },
                _ => {
                    panic!("Expected dyadic verb node, got: {:?}", ast);
                }
            }
        }
    }
    
    #[test]
    fn test_vector_operations() {
        let (result, _viz) = parse_and_visualize("1 2 3+4 5 6");
        assert!(result.is_ok());
        
        if let Ok(ast) = result {
            match ast {
                JNode::AmbiguousVerb(verb, left, right) => {
                    assert_eq!(verb, '+');
                    
                    // Verify it's parsing as dyadic with vector operands
                    if let (Some(left_node), Some(right_node)) = (left, right) {
                        match (*left_node, *right_node) {
                            (JNode::Literal(left_array), JNode::Literal(right_array)) => {
                                assert_eq!(left_array.data, vec![1, 2, 3]);
                                assert_eq!(right_array.data, vec![4, 5, 6]);
                            },
                            _ => panic!("Expected vector literals"),
                        }
                    }
                },
                _ => panic!("Expected dyadic verb for vector operation"),
            }
        }
    }
    
    #[test]
    fn test_build_succeeds() {
        // This test just verifies that LALRPOP compilation succeeded
        // and we can instantiate the parser without errors
        let parser = LalrParser::new();
        
        // This line will only compile if LALRPOP generated the parser successfully
        assert!(true); // Placeholder assertion - the real test is compilation
    }
}