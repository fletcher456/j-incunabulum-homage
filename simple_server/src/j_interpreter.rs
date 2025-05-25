// J Interpreter Module
// Basic implementation of a J interpreter with support for the iota verb

use std::fmt;

// J array types
#[derive(Debug, Clone)]
pub enum JType {
    Integer,
    Box,
}

// J array structure (similar to the C version's 'A' struct)
#[derive(Debug, Clone)]
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

// Interpreter struct to manage the J session
pub struct JInterpreter {
    // We could add symbol tables and other state here
}

impl JInterpreter {
    // Create a new interpreter
    pub fn new() -> Self {
        JInterpreter {}
    }

    // Iota verb: Generate a sequence of integers from 0 to n-1
    pub fn iota(&self, n: i64) -> Result<JArray, String> {
        if n < 0 {
            return Err("Domain error: iota requires a non-negative argument".to_string());
        }
        
        let n_usize = n as usize;
        let data: Vec<i64> = (0..n).collect();
        
        Ok(JArray::new_integer(1, vec![n_usize], data))
    }

    // Parse and execute a J expression
    pub fn execute(&self, input: &str) -> Result<JArray, String> {
        let input = input.trim();
        
        // Help command
        if input == "help" {
            return Err("Available commands:\n\
                       ~n - iota: generate integers from 0 to n-1\n\
                       help - show this help message".to_string());
        }
        
        // Handle iota verb
        if input.starts_with('~') {
            let arg = input[1..].trim();
            match arg.parse::<i64>() {
                Ok(n) => self.iota(n),
                Err(_) => Err(format!("Invalid argument for iota: '{}'", arg)),
            }
        } else if let Ok(n) = input.parse::<i64>() {
            // Just a number, return it as a scalar
            Ok(JArray::new_scalar(n))
        } else {
            Err(format!("Unsupported J expression: '{}'", input))
        }
    }
}

// Create a more user-friendly output for the web display
pub fn format_result(result: Result<JArray, String>) -> String {
    match result {
        Ok(array) => format!("{}", array),
        Err(error) => format!("Error: {}", error),
    }
}