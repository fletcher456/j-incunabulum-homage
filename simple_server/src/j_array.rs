// J Array Data Structure Module - Enhanced Implementation
// Core data types and operations for J language arrays with full multi-dimensional support

use std::fmt;

// Phase 1: Multi-Dimensional Array Support
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
        // J language rules for shape compatibility
        self.total_elements() == other.total_elements()
    }
}

// Phase 2: Enhanced Value Type System
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

impl fmt::Display for JValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JValue::Integer(i) => write!(f, "{}", i),
            JValue::Float(fl) => write!(f, "{}", fl),
            JValue::Character(c) => write!(f, "{}", c),
            JValue::Box(boxed) => write!(f, "<{}>", boxed),
        }
    }
}

// Phase 3: Error Handling System
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

// Enhanced JArray Structure
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
    
    // Phase 4: Advanced Array Operations
    
    // Reshape Implementation for # Operator
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
    
    // Indexing Implementation for { Operator
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
    
    // Concatenation Implementation for , Operator
    pub fn concatenate(&self, other: &JArray) -> Result<JArray, ArrayError> {
        // For vectors, simple concatenation
        if self.is_vector() && other.is_vector() {
            let mut result_data = self.data.clone();
            result_data.extend(other.data.clone());
            let result_len = result_data.len();
            
            Ok(JArray {
                data: result_data,
                shape: ArrayShape::vector(result_len),
            })
        } else if self.is_scalar() && other.is_scalar() {
            // Concatenate scalars into vector
            let mut result_data = vec![self.data[0].clone()];
            result_data.push(other.data[0].clone());
            
            Ok(JArray {
                data: result_data,
                shape: ArrayShape::vector(2),
            })
        } else {
            // For now, convert to vectors and concatenate
            let self_ravel = self.ravel();
            let other_ravel = other.ravel();
            self_ravel.concatenate(&other_ravel)
        }
    }
    
    pub fn ravel(&self) -> JArray {
        JArray {
            data: self.data.clone(),
            shape: ArrayShape::vector(self.data.len()),
        }
    }
    
    // Boxing Support for < Operator
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
    
    // Backward Compatibility Layer
    pub fn from_vec(data: Vec<i32>) -> Self {
        Self::vector(data)
    }
    
    pub fn get_data(&self) -> Vec<i32> {
        self.data.iter()
            .filter_map(|v| v.to_integer())
            .collect()
    }
    
    // Legacy constructors for existing code
    pub fn new(data: Vec<i32>) -> Self {
        Self::vector(data)
    }
    
    pub fn new_integer(rank: usize, shape: Vec<usize>, data: Vec<i64>) -> Self {
        let values: Vec<i32> = data.into_iter().map(|x| x as i32).collect();
        match rank {
            0 => Self::scalar(values[0]),
            1 => Self::vector(values),
            2 => Self::matrix(values, shape[0], shape[1]),
            _ => {
                // For higher dimensions, store as vector for now
                Self::vector(values)
            }
        }
    }
    
    pub fn new_scalar(value: i64) -> Self {
        Self::scalar(value as i32)
    }
}

// Phase 5: Display and Formatting
impl fmt::Display for JArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
                // Matrix with proper alignment
                let rows = self.shape.dimensions[0];
                let cols = self.shape.dimensions[1];
                
                // Find the maximum width needed for any number
                let max_width = self.data.iter()
                    .map(|v| format!("{}", v).len())
                    .max()
                    .unwrap_or(1);
                
                for row in 0..rows {
                    for col in 0..cols {
                        let index = row * cols + col;
                        let formatted_num = format!("{}", self.data[index]);
                        let padded_num = format!("{: >width$}", formatted_num, width = max_width);
                        
                        if col == 0 {
                            // First number in row
                            write!(f, "{}", padded_num)?;
                        } else {
                            // Subsequent numbers with space separator
                            write!(f, " {}", padded_num)?;
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

// Legacy enum for backward compatibility
#[derive(Debug, Clone, PartialEq)]
pub enum JType {
    Integer,
    Box,
}