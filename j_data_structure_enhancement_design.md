# J Data Structure Enhancement Design Document

## Overview

This document provides a comprehensive breakdown of all data structure changes required to support advanced J language operators, particularly focusing on multi-dimensional arrays, boxing, and enhanced type systems needed for operators like `#` (reshape), `{` (indexing), `,` (concatenate), and `<` (box).

## Current Data Structure Analysis

### Existing `JArray` Structure
```rust
// Current implementation in j_array.rs
pub struct JArray {
    pub data: Vec<i32>,
    pub array_type: JType,
}

pub enum JType {
    Integer,
    Box,  // Currently unused
}
```

### Current Limitations
1. **Single Dimension Only**: Only supports 1D vectors
2. **Integer Only**: No support for nested/boxed arrays
3. **No Shape Information**: Cannot represent matrices or higher dimensions
4. **No Type Diversity**: Limited to integers only

## Enhanced Data Structure Design

### Phase 1: Multi-Dimensional Array Support

#### New Shape System
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayShape {
    pub dimensions: Vec<usize>,
}

impl ArrayShape {
    pub fn scalar() -> Self {
        ArrayShape { dimensions: vec![] }
    }
    
    pub fn vector(len: usize) -> Self {
        ArrayShape { dimensions: vec![len] }
    }
    
    pub fn matrix(rows: usize, cols: usize) -> Self {
        ArrayShape { dimensions: vec![rows, cols] }
    }
    
    pub fn rank(&self) -> usize {
        self.dimensions.len()
    }
    
    pub fn total_elements(&self) -> usize {
        if self.dimensions.is_empty() {
            1 // scalar
        } else {
            self.dimensions.iter().product()
        }
    }
    
    pub fn is_compatible_with(&self, other: &ArrayShape) -> bool {
        // Implementation for broadcast compatibility
        // J language rules for shape compatibility
    }
}
```

#### Enhanced Array Structure
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct JArray {
    pub data: Vec<JValue>,
    pub shape: ArrayShape,
}

impl JArray {
    pub fn scalar(value: i32) -> Self {
        JArray {
            data: vec![JValue::Integer(value)],
            shape: ArrayShape::scalar(),
        }
    }
    
    pub fn vector(values: Vec<i32>) -> Self {
        let len = values.len();
        JArray {
            data: values.into_iter().map(JValue::Integer).collect(),
            shape: ArrayShape::vector(len),
        }
    }
    
    pub fn matrix(values: Vec<i32>, rows: usize, cols: usize) -> Self {
        assert_eq!(values.len(), rows * cols);
        JArray {
            data: values.into_iter().map(JValue::Integer).collect(),
            shape: ArrayShape::matrix(rows, cols),
        }
    }
    
    pub fn is_scalar(&self) -> bool {
        self.shape.rank() == 0
    }
    
    pub fn is_vector(&self) -> bool {
        self.shape.rank() == 1
    }
    
    pub fn is_matrix(&self) -> bool {
        self.shape.rank() == 2
    }
    
    pub fn get_at_index(&self, indices: &[usize]) -> Option<&JValue> {
        let flat_index = self.calculate_flat_index(indices)?;
        self.data.get(flat_index)
    }
    
    pub fn set_at_index(&mut self, indices: &[usize], value: JValue) -> Result<(), ArrayError> {
        let flat_index = self.calculate_flat_index(indices)
            .ok_or(ArrayError::IndexOutOfBounds)?;
        if let Some(elem) = self.data.get_mut(flat_index) {
            *elem = value;
            Ok(())
        } else {
            Err(ArrayError::IndexOutOfBounds)
        }
    }
    
    fn calculate_flat_index(&self, indices: &[usize]) -> Option<usize> {
        if indices.len() != self.shape.rank() {
            return None;
        }
        
        let mut flat_index = 0;
        let mut multiplier = 1;
        
        for i in (0..indices.len()).rev() {
            if indices[i] >= self.shape.dimensions[i] {
                return None;
            }
            flat_index += indices[i] * multiplier;
            multiplier *= self.shape.dimensions[i];
        }
        
        Some(flat_index)
    }
}
```

### Phase 2: Enhanced Value Type System

#### Comprehensive Value Types
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum JValue {
    Integer(i32),
    Float(f64),
    Character(char),
    Box(Box<JArray>),
}

impl JValue {
    pub fn type_name(&self) -> &'static str {
        match self {
            JValue::Integer(_) => "integer",
            JValue::Float(_) => "float",
            JValue::Character(_) => "character",
            JValue::Box(_) => "box",
        }
    }
    
    pub fn is_numeric(&self) -> bool {
        matches!(self, JValue::Integer(_) | JValue::Float(_))
    }
    
    pub fn to_integer(&self) -> Option<i32> {
        match self {
            JValue::Integer(i) => Some(*i),
            JValue::Float(f) => Some(*f as i32),
            _ => None,
        }
    }
    
    pub fn to_float(&self) -> Option<f64> {
        match self {
            JValue::Integer(i) => Some(*i as f64),
            JValue::Float(f) => Some(*f),
            _ => None,
        }
    }
}
```

#### Boxing Support for `<` Operator
```rust
impl JArray {
    pub fn box_array(array: JArray) -> Self {
        JArray {
            data: vec![JValue::Box(Box::new(array))],
            shape: ArrayShape::scalar(),
        }
    }
    
    pub fn unbox(&self) -> Option<&JArray> {
        if self.is_scalar() {
            if let JValue::Box(boxed) = &self.data[0] {
                Some(boxed.as_ref())
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub fn is_boxed(&self) -> bool {
        self.is_scalar() && matches!(self.data[0], JValue::Box(_))
    }
}
```

### Phase 3: Error Handling System

#### Comprehensive Error Types
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ArrayError {
    IndexOutOfBounds,
    ShapeMismatch { expected: ArrayShape, actual: ArrayShape },
    TypeMismatch { expected: String, actual: String },
    InvalidReshape { from_shape: ArrayShape, to_shape: ArrayShape },
    DivisionByZero,
    EmptyArray,
    InvalidDimension(usize),
}

impl std::fmt::Display for ArrayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrayError::IndexOutOfBounds => write!(f, "Index out of bounds"),
            ArrayError::ShapeMismatch { expected, actual } => {
                write!(f, "Shape mismatch: expected {:?}, got {:?}", expected, actual)
            }
            ArrayError::TypeMismatch { expected, actual } => {
                write!(f, "Type mismatch: expected {}, got {}", expected, actual)
            }
            ArrayError::InvalidReshape { from_shape, to_shape } => {
                write!(f, "Cannot reshape from {:?} to {:?}", from_shape, to_shape)
            }
            ArrayError::DivisionByZero => write!(f, "Division by zero"),
            ArrayError::EmptyArray => write!(f, "Operation on empty array"),
            ArrayError::InvalidDimension(dim) => write!(f, "Invalid dimension: {}", dim),
        }
    }
}

impl std::error::Error for ArrayError {}
```

### Phase 4: Advanced Array Operations

#### Reshape Implementation for `#` Operator
```rust
impl JArray {
    pub fn reshape(&self, new_shape: ArrayShape) -> Result<JArray, ArrayError> {
        let current_elements = self.shape.total_elements();
        let new_elements = new_shape.total_elements();
        
        if current_elements != new_elements {
            return Err(ArrayError::InvalidReshape {
                from_shape: self.shape.clone(),
                to_shape: new_shape,
            });
        }
        
        Ok(JArray {
            data: self.data.clone(),
            shape: new_shape,
        })
    }
    
    pub fn tally(&self) -> usize {
        if self.shape.dimensions.is_empty() {
            1 // scalar has tally 1
        } else {
            self.shape.dimensions[0] // first dimension
        }
    }
}
```

#### Indexing Implementation for `{` Operator
```rust
impl JArray {
    pub fn select_from(&self, indices: &JArray) -> Result<JArray, ArrayError> {
        let mut result_data = Vec::new();
        
        for index_value in &indices.data {
            let index = index_value.to_integer()
                .ok_or(ArrayError::TypeMismatch {
                    expected: "integer".to_string(),
                    actual: index_value.type_name().to_string(),
                })?;
            
            if index < 0 || index as usize >= self.data.len() {
                return Err(ArrayError::IndexOutOfBounds);
            }
            
            result_data.push(self.data[index as usize].clone());
        }
        
        Ok(JArray {
            data: result_data,
            shape: indices.shape.clone(),
        })
    }
}
```

#### Concatenation Implementation for `,` Operator
```rust
impl JArray {
    pub fn concatenate(&self, other: &JArray) -> Result<JArray, ArrayError> {
        // For vectors, simple concatenation
        if self.is_vector() && other.is_vector() {
            let mut result_data = self.data.clone();
            result_data.extend(other.data.clone());
            
            Ok(JArray {
                data: result_data,
                shape: ArrayShape::vector(result_data.len()),
            })
        } else {
            // For matrices and higher dimensions, concatenate along first axis
            self.concatenate_along_axis(other, 0)
        }
    }
    
    pub fn concatenate_along_axis(&self, other: &JArray, axis: usize) -> Result<JArray, ArrayError> {
        if self.shape.rank() != other.shape.rank() {
            return Err(ArrayError::ShapeMismatch {
                expected: self.shape.clone(),
                actual: other.shape.clone(),
            });
        }
        
        // Check that all dimensions except the concatenation axis match
        for (i, (dim1, dim2)) in self.shape.dimensions.iter()
            .zip(other.shape.dimensions.iter()).enumerate() {
            if i != axis && dim1 != dim2 {
                return Err(ArrayError::ShapeMismatch {
                    expected: self.shape.clone(),
                    actual: other.shape.clone(),
                });
            }
        }
        
        // Implementation for concatenation along specified axis
        // Complex logic for multi-dimensional arrays
        todo!("Implement multi-dimensional concatenation")
    }
    
    pub fn ravel(&self) -> JArray {
        JArray {
            data: self.data.clone(),
            shape: ArrayShape::vector(self.data.len()),
        }
    }
}
```

### Phase 5: Display and Formatting

#### Enhanced Display Implementation
```rust
impl std::fmt::Display for JArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.shape.rank() {
            0 => {
                // Scalar
                write!(f, "{}", self.data[0])
            }
            1 => {
                // Vector
                let values: Vec<String> = self.data.iter()
                    .map(|v| format!("{}", v))
                    .collect();
                write!(f, "{}", values.join(" "))
            }
            2 => {
                // Matrix
                let rows = self.shape.dimensions[0];
                let cols = self.shape.dimensions[1];
                
                for row in 0..rows {
                    for col in 0..cols {
                        let index = row * cols + col;
                        write!(f, "{}", self.data[index])?;
                        if col < cols - 1 {
                            write!(f, " ")?;
                        }
                    }
                    if row < rows - 1 {
                        writeln!(f)?;
                    }
                }
                Ok(())
            }
            _ => {
                // Higher dimensions - flatten representation
                write!(f, "{}", self.ravel())
            }
        }
    }
}

impl std::fmt::Display for JValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JValue::Integer(i) => write!(f, "{}", i),
            JValue::Float(fl) => write!(f, "{}", fl),
            JValue::Character(c) => write!(f, "{}", c),
            JValue::Box(boxed) => write!(f, "<{}>", boxed),
        }
    }
}
```

## Migration Strategy

### Step 1: Backward Compatibility Layer
```rust
// Maintain compatibility with existing code
impl JArray {
    // Legacy constructor for existing tests
    pub fn from_vec(data: Vec<i32>) -> Self {
        Self::vector(data)
    }
    
    // Legacy getter for existing evaluator
    pub fn get_data(&self) -> Vec<i32> {
        self.data.iter()
            .filter_map(|v| v.to_integer())
            .collect()
    }
}
```

### Step 2: Gradual Operator Updates
1. Update `+` and `~` operators to work with new structure
2. Add `#` operator with basic reshape functionality
3. Add `,` operator with concatenation
4. Add `<` operator with boxing
5. Add `{` operator with indexing

### Step 3: Testing Infrastructure
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_creation() {
        let scalar = JArray::scalar(42);
        assert!(scalar.is_scalar());
        assert_eq!(scalar.data[0], JValue::Integer(42));
    }

    #[test]
    fn test_matrix_reshape() {
        let vector = JArray::vector(vec![1, 2, 3, 4, 5, 6]);
        let matrix = vector.reshape(ArrayShape::matrix(2, 3)).unwrap();
        assert!(matrix.is_matrix());
        assert_eq!(matrix.shape.dimensions, vec![2, 3]);
    }

    #[test]
    fn test_indexing() {
        let array = JArray::vector(vec![10, 20, 30, 40]);
        let indices = JArray::vector(vec![0, 2]);
        let result = array.select_from(&indices).unwrap();
        assert_eq!(result.get_data(), vec![10, 30]);
    }

    #[test]
    fn test_boxing() {
        let array = JArray::vector(vec![1, 2, 3]);
        let boxed = JArray::box_array(array.clone());
        assert!(boxed.is_boxed());
        assert_eq!(boxed.unbox().unwrap(), &array);
    }
}
```

## Implementation Timeline

### Week 1: Foundation
- Implement `ArrayShape` structure
- Implement `JValue` enum
- Update `JArray` with shape support
- Add error handling system

### Week 2: Core Operations
- Implement reshape functionality
- Add indexing operations
- Implement concatenation
- Add boxing support

### Week 3: Integration
- Update tokenizer (if needed)
- Update semantic analyzer
- Update evaluator with new operations
- Comprehensive testing

### Week 4: Polish
- Performance optimization
- Error message improvements
- Documentation updates
- Edge case handling

## Success Criteria

### Functional Requirements
- ✅ All existing expressions continue to work
- ✅ `#` operator works for tally and reshape
- ✅ `{` operator works for indexing
- ✅ `,` operator works for concatenation and ravel
- ✅ `<` operator works for boxing

### Quality Requirements
- ✅ Comprehensive error handling
- ✅ Clear error messages
- ✅ Backward compatibility maintained
- ✅ Performance within acceptable bounds
- ✅ Memory usage optimized

### Testing Requirements
- ✅ Unit tests for all new functionality
- ✅ Integration tests with parser
- ✅ Edge case testing
- ✅ Performance benchmarks

This design provides a robust foundation for implementing all remaining J language operators while maintaining the excellent architecture and parsing capabilities already established with LALRPOP.