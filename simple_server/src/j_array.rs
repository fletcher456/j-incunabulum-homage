// J Array Data Structure Module
// Core data types and operations for J language arrays

use std::fmt;

// J array types
#[derive(Debug, Clone, PartialEq)]
pub enum JType {
    Integer,
    Box,
}

// J array structure (unified representation for scalars and arrays)
#[derive(Debug, Clone, PartialEq)]
pub struct JArray {
    pub array_type: JType,
    pub rank: usize,
    pub shape: Vec<usize>,
    pub data: Vec<i64>,
}

impl JArray {
    // Create a new integer array
    pub fn new_integer(rank: usize, shape: Vec<usize>, data: Vec<i64>) -> Self {
        JArray {
            array_type: JType::Integer,
            rank,
            shape,
            data,
        }
    }

    // Create a scalar integer
    pub fn new_scalar(value: i64) -> Self {
        JArray {
            array_type: JType::Integer,
            rank: 0,
            shape: vec![],
            data: vec![value],
        }
    }

    // Calculate the total number of elements in the array
    pub fn tally(&self) -> usize {
        if self.rank == 0 {
            1
        } else {
            self.shape.iter().product()
        }
    }
}

// Display implementation for JArray to format output
impl fmt::Display for JArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.array_type {
            JType::Integer => {
                if self.rank == 0 {
                    // Scalar
                    write!(f, "{}", self.data[0])
                } else if self.rank == 1 {
                    // Vector
                    write!(
                        f,
                        "{}",
                        self.data
                            .iter()
                            .map(|i| i.to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                } else {
                    // Matrix or higher dimensions - simplified output
                    write!(f, "Array(rank={}, shape={:?}): {:?}", self.rank, self.shape, self.data)
                }
            }
            JType::Box => {
                write!(f, "<box>")
            }
        }
    }
}