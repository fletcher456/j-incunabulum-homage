// Comprehensive Test Suite for Enhanced J Data Structures
// Testing all five phases of implementation

#[cfg(test)]
mod tests {
    use crate::j_array::{JArray, JValue, ArrayShape, ArrayError};
    use crate::semantic_analyzer::JSemanticAnalyzer;
    use crate::evaluator::JEvaluator;
    use crate::parser::JNode;

    // Phase 1 Tests: Multi-Dimensional Array Support
    #[test]
    fn test_array_shape_creation() {
        let scalar_shape = ArrayShape::scalar();
        assert_eq!(scalar_shape.rank(), 0);
        assert_eq!(scalar_shape.total_elements(), 1);

        let vector_shape = ArrayShape::vector(5);
        assert_eq!(vector_shape.rank(), 1);
        assert_eq!(vector_shape.total_elements(), 5);

        let matrix_shape = ArrayShape::matrix(2, 3);
        assert_eq!(matrix_shape.rank(), 2);
        assert_eq!(matrix_shape.total_elements(), 6);
    }

    #[test]
    fn test_array_creation() {
        let scalar = JArray::scalar(42);
        assert!(scalar.is_scalar());
        assert_eq!(scalar.data[0], JValue::Integer(42));

        let vector = JArray::vector(vec![1, 2, 3, 4]);
        assert!(vector.is_vector());
        assert_eq!(vector.shape.dimensions, vec![4]);

        let matrix = JArray::matrix(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert!(matrix.is_matrix());
        assert_eq!(matrix.shape.dimensions, vec![2, 3]);
    }

    // Phase 2 Tests: Enhanced Value Type System
    #[test]
    fn test_jvalue_types() {
        let int_val = JValue::Integer(42);
        assert_eq!(int_val.type_name(), "integer");
        assert!(int_val.is_numeric());
        assert_eq!(int_val.to_integer(), Some(42));

        let float_val = JValue::Float(3.14);
        assert_eq!(float_val.type_name(), "float");
        assert!(float_val.is_numeric());
        assert_eq!(float_val.to_float(), Some(3.14));

        let char_val = JValue::Character('A');
        assert_eq!(char_val.type_name(), "character");
        assert!(!char_val.is_numeric());
    }

    #[test]
    fn test_boxing() {
        let array = JArray::vector(vec![1, 2, 3]);
        let boxed = JArray::box_array(array.clone());
        
        assert!(boxed.is_boxed());
        assert!(boxed.is_scalar());
        assert_eq!(boxed.unbox().unwrap(), &array);
    }

    // Phase 3 Tests: Error Handling System
    #[test]
    fn test_reshape_errors() {
        let array = JArray::vector(vec![1, 2, 3, 4]);
        let invalid_shape = ArrayShape::vector(5); // Wrong number of elements
        
        let result = array.reshape(invalid_shape);
        assert!(result.is_err());
        
        if let Err(ArrayError::InvalidReshape { from_shape, to_shape }) = result {
            assert_eq!(from_shape.total_elements(), 4);
            assert_eq!(to_shape.total_elements(), 5);
        } else {
            panic!("Expected InvalidReshape error");
        }
    }

    #[test]
    fn test_index_bounds_errors() {
        let array = JArray::vector(vec![1, 2, 3]);
        let bad_indices = JArray::vector(vec![0, 5]); // Index 5 is out of bounds
        
        let result = array.select_from(&bad_indices);
        assert!(result.is_err());
    }

    // Phase 4 Tests: Advanced Array Operations
    #[test]
    fn test_reshape_operation() {
        let vector = JArray::vector(vec![1, 2, 3, 4, 5, 6]);
        let matrix_shape = ArrayShape::matrix(2, 3);
        
        let matrix = vector.reshape(matrix_shape).unwrap();
        assert!(matrix.is_matrix());
        assert_eq!(matrix.shape.dimensions, vec![2, 3]);
        assert_eq!(matrix.get_data(), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_tally_operation() {
        let scalar = JArray::scalar(42);
        assert_eq!(scalar.tally(), 1);

        let vector = JArray::vector(vec![1, 2, 3, 4, 5]);
        assert_eq!(vector.tally(), 5);

        let matrix = JArray::matrix(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(matrix.tally(), 2); // First dimension
    }

    #[test]
    fn test_indexing_operation() {
        let source = JArray::vector(vec![10, 20, 30, 40, 50]);
        let indices = JArray::vector(vec![0, 2, 4]);
        
        let result = source.select_from(&indices).unwrap();
        assert_eq!(result.get_data(), vec![10, 30, 50]);
    }

    #[test]
    fn test_concatenation() {
        let left = JArray::vector(vec![1, 2, 3]);
        let right = JArray::vector(vec![4, 5, 6]);
        
        let result = left.concatenate(&right).unwrap();
        assert_eq!(result.get_data(), vec![1, 2, 3, 4, 5, 6]);

        // Test scalar concatenation
        let scalar1 = JArray::scalar(1);
        let scalar2 = JArray::scalar(2);
        let scalar_result = scalar1.concatenate(&scalar2).unwrap();
        assert_eq!(scalar_result.get_data(), vec![1, 2]);
    }

    #[test]
    fn test_ravel_operation() {
        let matrix = JArray::matrix(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let raveled = matrix.ravel();
        
        assert!(raveled.is_vector());
        assert_eq!(raveled.get_data(), vec![1, 2, 3, 4, 5, 6]);
    }

    // Phase 5 Tests: Display and Formatting
    #[test]
    fn test_display_formatting() {
        let scalar = JArray::scalar(42);
        assert_eq!(format!("{}", scalar), "42");

        let vector = JArray::vector(vec![1, 2, 3]);
        assert_eq!(format!("{}", vector), "1 2 3");

        let matrix = JArray::matrix(vec![1, 2, 3, 4], 2, 2);
        let matrix_display = format!("{}", matrix);
        assert!(matrix_display.contains("1 2"));
        assert!(matrix_display.contains("3 4"));
    }

    // Integration Tests: Full Operator Pipeline
    #[test]
    fn test_plus_operator_integration() {
        let evaluator = JEvaluator::new();

        // Test monadic plus (identity)
        let array = JArray::vector(vec![1, 2, 3]);
        let literal_node = JNode::Literal(array.clone());
        let plus_node = JNode::MonadicVerb('+', Box::new(literal_node));
        
        let result = evaluator.evaluate(&plus_node).unwrap();
        assert_eq!(result.get_data(), vec![1, 2, 3]);

        // Test dyadic plus
        let left = JNode::Literal(JArray::scalar(5));
        let right = JNode::Literal(JArray::vector(vec![1, 2, 3]));
        let plus_dyadic = JNode::DyadicVerb('+', Box::new(left), Box::new(right));
        
        let result = evaluator.evaluate(&plus_dyadic).unwrap();
        assert_eq!(result.get_data(), vec![6, 7, 8]);
    }

    #[test]
    fn test_iota_operator() {
        let evaluator = JEvaluator::new();
        
        let five = JNode::Literal(JArray::scalar(5));
        let iota_node = JNode::MonadicVerb('~', Box::new(five));
        
        let result = evaluator.evaluate(&iota_node).unwrap();
        assert_eq!(result.get_data(), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_tally_operator() {
        let evaluator = JEvaluator::new();
        
        let vector = JNode::Literal(JArray::vector(vec![1, 2, 3, 4, 5]));
        let tally_node = JNode::MonadicVerb('#', Box::new(vector));
        
        let result = evaluator.evaluate(&tally_node).unwrap();
        assert_eq!(result.get_data(), vec![5]);
    }

    #[test]
    fn test_reshape_operator() {
        let evaluator = JEvaluator::new();
        
        let shape = JNode::Literal(JArray::vector(vec![2, 3]));
        let data = JNode::Literal(JArray::vector(vec![1, 2, 3, 4, 5, 6]));
        let reshape_node = JNode::DyadicVerb('#', Box::new(shape), Box::new(data));
        
        let result = evaluator.evaluate(&reshape_node).unwrap();
        assert!(result.is_matrix());
        assert_eq!(result.shape.dimensions, vec![2, 3]);
        assert_eq!(result.get_data(), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_from_operator() {
        let evaluator = JEvaluator::new();
        
        let indices = JNode::Literal(JArray::vector(vec![0, 2]));
        let source = JNode::Literal(JArray::vector(vec![10, 20, 30, 40]));
        let from_node = JNode::DyadicVerb('{', Box::new(indices), Box::new(source));
        
        let result = evaluator.evaluate(&from_node).unwrap();
        assert_eq!(result.get_data(), vec![10, 30]);
    }

    #[test]
    fn test_concatenate_operator() {
        let evaluator = JEvaluator::new();
        
        let left = JNode::Literal(JArray::vector(vec![1, 2]));
        let right = JNode::Literal(JArray::vector(vec![3, 4]));
        let concat_node = JNode::DyadicVerb(',', Box::new(left), Box::new(right));
        
        let result = evaluator.evaluate(&concat_node).unwrap();
        assert_eq!(result.get_data(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_ravel_operator() {
        let evaluator = JEvaluator::new();
        
        let matrix = JNode::Literal(JArray::matrix(vec![1, 2, 3, 4], 2, 2));
        let ravel_node = JNode::MonadicVerb(',', Box::new(matrix));
        
        let result = evaluator.evaluate(&ravel_node).unwrap();
        assert!(result.is_vector());
        assert_eq!(result.get_data(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_box_operator() {
        let evaluator = JEvaluator::new();
        
        let array = JNode::Literal(JArray::vector(vec![1, 2, 3]));
        let box_node = JNode::MonadicVerb('<', Box::new(array));
        
        let result = evaluator.evaluate(&box_node).unwrap();
        assert!(result.is_boxed());
        assert!(result.is_scalar());
    }

    #[test]
    fn test_less_than_operator() {
        let evaluator = JEvaluator::new();
        
        let left = JNode::Literal(JArray::scalar(3));
        let right = JNode::Literal(JArray::scalar(5));
        let lt_node = JNode::DyadicVerb('<', Box::new(left), Box::new(right));
        
        let result = evaluator.evaluate(&lt_node).unwrap();
        assert_eq!(result.get_data(), vec![1]); // 3 < 5 is true (1)

        // Test false case
        let left2 = JNode::Literal(JArray::scalar(5));
        let right2 = JNode::Literal(JArray::scalar(3));
        let lt_node2 = JNode::DyadicVerb('<', Box::new(left2), Box::new(right2));
        
        let result2 = evaluator.evaluate(&lt_node2).unwrap();
        assert_eq!(result2.get_data(), vec![0]); // 5 < 3 is false (0)
    }

    // Backward Compatibility Tests
    #[test]
    fn test_backward_compatibility() {
        // Test legacy constructors still work
        let legacy_array = JArray::new(vec![1, 2, 3, 4]);
        assert_eq!(legacy_array.get_data(), vec![1, 2, 3, 4]);

        let legacy_scalar = JArray::new_scalar(42);
        assert_eq!(legacy_scalar.get_data(), vec![42]);

        let legacy_integer = JArray::new_integer(1, vec![3], vec![1, 2, 3]);
        assert_eq!(legacy_integer.get_data(), vec![1, 2, 3]);
    }

    // Complex Expression Tests
    #[test]
    fn test_complex_expression() {
        let evaluator = JEvaluator::new();
        
        // Test: #~5 (tally of iota 5, should be 5)
        let five = JNode::Literal(JArray::scalar(5));
        let iota_five = JNode::MonadicVerb('~', Box::new(five));
        let tally_iota_five = JNode::MonadicVerb('#', Box::new(iota_five));
        
        let result = evaluator.evaluate(&tally_iota_five).unwrap();
        assert_eq!(result.get_data(), vec![5]);
    }

    #[test]
    fn test_precedence_with_new_operators() {
        // This would test expressions like ~3+#~4
        // Where # has higher precedence than +
        let evaluator = JEvaluator::new();
        
        // Create ~3 + (#~4)
        let three = JNode::Literal(JArray::scalar(3));
        let iota_three = JNode::MonadicVerb('~', Box::new(three));
        
        let four = JNode::Literal(JArray::scalar(4));
        let iota_four = JNode::MonadicVerb('~', Box::new(four));
        let tally_iota_four = JNode::MonadicVerb('#', Box::new(iota_four));
        
        let plus_expr = JNode::DyadicVerb('+', Box::new(iota_three), Box::new(tally_iota_four));
        
        let result = evaluator.evaluate(&plus_expr).unwrap();
        // ~3 is [0,1,2], #~4 is 4, so result should be [4,5,6]
        assert_eq!(result.get_data(), vec![4, 5, 6]);
    }
}