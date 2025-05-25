use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::io::Write;
use libc::{malloc, strlen};
use std::ptr;

// Direct translations of C types
pub type C = i8;  // char in C
pub type I = i64; // long in C

// Direct translation of struct a
#[repr(C)]
pub struct a {
    pub t: I,
    pub r: I,
    pub d: [I; 3],
    pub p: [I; 2],
}

// Type A is a pointer to struct a
pub type A = *mut a;

// Memory allocation - direct translation of ma(n)
#[no_mangle]
pub unsafe extern "C" fn ma(n: I) -> *mut I {
    (malloc((n * 4) as usize) as *mut I)
}

// Memory copy - direct translation of mv(d,s,n)
#[no_mangle]
pub unsafe extern "C" fn mv(d: *mut I, s: *mut I, n: I) {
    let mut i = 0;
    while i < n {
        *d.offset(i as isize) = *s.offset(i as isize);
        i += 1;
    }
}

// Calculate total size - direct translation of tr(r,d)
#[no_mangle]
pub unsafe extern "C" fn tr(r: I, d: *mut I) -> I {
    let mut z: I = 1;
    let mut i = 0;
    while i < r {
        z = z * *d.offset(i as isize);
        i += 1;
    }
    z
}

// Create array - direct translation of ga(t,r,d)
#[no_mangle]
pub unsafe extern "C" fn ga(t: I, r: I, d: *mut I) -> A {
    let z = ma(5 + tr(r, d)) as A;
    (*z).t = t;
    (*z).r = r;
    mv((*z).d.as_mut_ptr(), d, r);
    z
}

// Iota function - direct translation of iota(w)
#[no_mangle]
pub unsafe extern "C" fn iota(w: A) -> A {
    let mut n = *(*w).p.as_ptr();
    let z = ga(0, 1, &mut n);
    let mut i = 0;
    while i < n {
        *(*z).p.as_mut_ptr().offset(i as isize) = i;
        i += 1;
    }
    z
}

// Plus function - direct translation of plus(a,w)
#[no_mangle]
pub unsafe extern "C" fn plus(a_ptr: A, w: A) -> A {
    let r = (*w).r;
    let d = (*w).d.as_mut_ptr();
    let n = tr(r, d);
    let z = ga(0, r, d);
    let mut i = 0;
    while i < n {
        *(*z).p.as_mut_ptr().offset(i as isize) = 
            *(*a_ptr).p.as_mut_ptr().offset(i as isize) + 
            *(*w).p.as_mut_ptr().offset(i as isize);
        i += 1;
    }
    z
}

// From function - direct translation of from(a,w)
#[no_mangle]
pub unsafe extern "C" fn from(a_ptr: A, w: A) -> A {
    let r = (*w).r - 1;
    let d = (*w).d.as_mut_ptr().offset(1);
    let n = tr(r, d);
    let z = ga((*w).t, r, d);
    mv(
        (*z).p.as_mut_ptr(), 
        (*w).p.as_mut_ptr().offset((n * *(*a_ptr).p.as_ptr()) as isize),
        n
    );
    z
}

// Box function - direct translation of box(w)
#[no_mangle]
pub unsafe extern "C" fn box_func(w: A) -> A {
    let z = ga(1, 0, ptr::null_mut());
    *(*z).p.as_mut_ptr() = w as I;
    z
}

// Cat function - direct translation of cat(a,w)
#[no_mangle]
pub unsafe extern "C" fn cat(a_ptr: A, w: A) -> A {
    let an = tr((*a_ptr).r, (*a_ptr).d.as_mut_ptr());
    let wn = tr((*w).r, (*w).d.as_mut_ptr());
    let mut n = an + wn;
    let z = ga((*w).t, 1, &mut n);
    mv((*z).p.as_mut_ptr(), (*a_ptr).p.as_mut_ptr(), an);
    mv((*z).p.as_mut_ptr().offset(an as isize), (*w).p.as_mut_ptr(), wn);
    z
}

// Find function - empty implementation as in the fragment
#[no_mangle]
pub unsafe extern "C" fn find(_a: A, _w: A) -> A {
    ptr::null_mut()
}

// Reshape function - direct translation of rsh(a,w)
#[no_mangle]
pub unsafe extern "C" fn rsh(a_ptr: A, w: A) -> A {
    let r = if (*a_ptr).r > 0 { *(*a_ptr).d.as_ptr() } else { 1 };
    let n = tr(r, (*a_ptr).p.as_mut_ptr());
    let wn = tr((*w).r, (*w).d.as_mut_ptr());
    let z = ga((*w).t, r, (*a_ptr).p.as_mut_ptr());
    let wn_adjusted = if n > wn { wn } else { n };
    mv((*z).p.as_mut_ptr(), (*w).p.as_mut_ptr(), wn_adjusted);
    let mut remaining = n - wn_adjusted;
    if remaining > 0 {
        mv(
            (*z).p.as_mut_ptr().offset(wn_adjusted as isize),
            (*z).p.as_mut_ptr(),
            remaining
        );
    }
    z
}

// Shape function - direct translation of sha(w)
#[no_mangle]
pub unsafe extern "C" fn sha(w: A) -> A {
    let mut w_r = (*w).r;
    let z = ga(0, 1, &mut w_r);
    mv((*z).p.as_mut_ptr(), (*w).d.as_mut_ptr(), (*w).r);
    z
}

// Identity function - direct translation of id(w)
#[no_mangle]
pub unsafe extern "C" fn id(w: A) -> A {
    w
}

// Size function - direct translation of size(w)
#[no_mangle]
pub unsafe extern "C" fn size(w: A) -> A {
    let z = ga(0, 0, ptr::null_mut());
    *(*z).p.as_mut_ptr() = if (*w).r > 0 { *(*w).d.as_ptr() } else { 1 };
    z
}

// Print integer - direct translation of pi(i)
#[no_mangle]
pub unsafe extern "C" fn pi(i: I) {
    print!("{} ", i);
}

// Print newline - direct translation of nl()
#[no_mangle]
pub unsafe extern "C" fn nl() {
    println!();
}

// Print array - direct translation of pr(w)
#[no_mangle]
pub unsafe extern "C" fn pr(w: A) -> I {
    let r = (*w).r;
    let d = (*w).d.as_mut_ptr();
    let n = tr(r, d);
    
    let mut i = 0;
    while i < r {
        pi(*d.offset(i as isize));
        i += 1;
    }
    nl();
    
    if (*w).t != 0 {
        let mut i = 0;
        while i < n {
            print!("< ");
            pr(*(*w).p.as_mut_ptr().offset(i as isize) as A);
            i += 1;
        }
    } else {
        let mut i = 0;
        while i < n {
            pi(*(*w).p.as_mut_ptr().offset(i as isize));
            i += 1;
        }
    }
    nl();
    0
}

// Verb table - direct translation of vt[]
static mut VT: [C; 7] = [b'+' as C, b'{' as C, b'~' as C, b'<' as C, b'#' as C, b',' as C, 0];

// Symbol table - direct translation of st[]
static mut ST: [I; 26] = [0; 26];

// Check if character is a variable name - direct translation of qp(a)
#[no_mangle]
pub unsafe extern "C" fn qp(a: I) -> I {
    if a >= 'a' as I && a <= 'z' as I { 1 } else { 0 }
}

// Check if character is a verb - direct translation of qv(a)
#[no_mangle]
pub unsafe extern "C" fn qv(a: I) -> I {
    if a < 'a' as I { 1 } else { 0 }
}

// We need function pointer types for the verb tables
type V1Func = unsafe extern "C" fn(A) -> A;
type V2Func = unsafe extern "C" fn(A, A) -> A;

// Global function pointer arrays
static mut VD: [Option<V2Func>; 7] = [None; 7];
static mut VM: [Option<V1Func>; 7] = [None; 7];

// Initialize the function pointer tables
fn init_function_tables() {
    unsafe {
        VD[0] = None;
        VD[1] = Some(plus);
        VD[2] = Some(from);
        VD[3] = Some(find);
        VD[4] = None;
        VD[5] = Some(rsh);
        VD[6] = Some(cat);
        
        VM[0] = None;
        VM[1] = Some(id);
        VM[2] = Some(size);
        VM[3] = Some(iota);
        VM[4] = Some(box_func);
        VM[5] = Some(sha);
        VM[6] = None;
    }
}

// Execute expression - direct translation of ex(e)
#[no_mangle]
pub unsafe extern "C" fn ex(e: *mut I) -> A {
    let mut a = *e;
    
    if qp(a) != 0 {
        if *e.offset(1) == '=' as I {
            let idx = (a - 'a' as I) as usize;
            if idx < ST.len() {
                ST[idx] = ex(e.offset(2)) as I;
                return ST[idx] as A;
            }
            return ptr::null_mut();
        }
        let idx = (a - 'a' as I) as usize;
        if idx < ST.len() {
            a = ST[idx];
        }
    }
    
    if qv(a) != 0 {
        let idx = a as usize;
        if idx < VM.len() && VM[idx].is_some() {
            let vm_fn = VM[idx].unwrap();
            return vm_fn(ex(e.offset(1)));
        }
        return ptr::null_mut();
    } else if *e.offset(1) != 0 {
        let idx = *e.offset(1) as usize;
        if idx < VD.len() && VD[idx].is_some() {
            let vd_fn = VD[idx].unwrap();
            return vd_fn(a as A, ex(e.offset(2)));
        }
        return ptr::null_mut();
    } else {
        return a as A;
    }
}

// Parse noun - direct translation of noun(c)
#[no_mangle]
pub unsafe extern "C" fn noun(c: C) -> A {
    if c < '0' as C || c > '9' as C {
        return ptr::null_mut();
    }
    
    let z = ga(0, 0, ptr::null_mut());
    *(*z).p.as_mut_ptr() = c as I - '0' as I;
    z
}

// Parse verb - direct translation of verb(c)
#[no_mangle]
pub unsafe extern "C" fn verb(c: C) -> I {
    let mut i: I = 0;
    while i < VT.len() as I && VT[i as usize] != 0 {
        if VT[i as usize] == c {
            return i + 1;
        }
        i += 1;
    }
    0
}

// Parse words - direct translation of wd(s)
#[no_mangle]
pub unsafe extern "C" fn wd(s: *const C) -> *mut I {
    let n = strlen(s as *const i8) as I;
    let e = ma(n + 1);
    
    let mut i = 0;
    while i < n {
        let c = *s.offset(i as isize);
        let a_noun = noun(c);
        if !a_noun.is_null() {
            *e.offset(i as isize) = a_noun as I;
        } else {
            let a_verb = verb(c);
            if a_verb != 0 {
                *e.offset(i as isize) = a_verb;
            } else {
                *e.offset(i as isize) = c as I;
            }
        }
        i += 1;
    }
    
    *e.offset(n as isize) = 0;
    e
}

// Capture output from print functions for our interpreter
struct OutputCapture {
    buffer: String,
}

impl OutputCapture {
    fn new() -> Self {
        OutputCapture {
            buffer: String::new(),
        }
    }
    
    fn append(&mut self, text: &str) {
        self.buffer.push_str(text);
    }
}

// Our main function to execute J code and return result
#[no_mangle]
pub unsafe extern "C" fn interpret_j_code(input: *const c_char) -> *mut c_char {
    // Initialize function tables if not already done
    init_function_tables();
    
    // Convert C string to Rust string
    let c_str = CStr::from_ptr(input);
    let rust_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => "Error: Invalid input string",
    };
    
    // Prepare a buffer for output
    let mut output = String::new();
    
    // Convert Rust string to C string for our J interpreter
    let c_input = rust_str.as_ptr() as *const C;
    
    // Execute the J code using the original interface
    let result = ex(wd(c_input));
    
    // Process the result and format it properly
    if !result.is_null() {
        // Format arrays and numbers in a more readable way
        if (*result).r == 0 {
            // Single value
            output = format!("{}", *(*result).p.as_ptr());
        } else {
            // Array result
            let n = tr((*result).r, (*result).d.as_mut_ptr());
            output.push_str("[");
            
            let mut i = 0;
            while i < n {
                if i > 0 {
                    output.push_str(" ");
                }
                output.push_str(&format!("{}", *(*result).p.as_mut_ptr().offset(i as isize)));
                i += 1;
            }
            
            output.push_str("]");
        }
    } else {
        output = "Error evaluating J expression".to_string();
    }
    
    // Return the result as a C string
    CString::new(output).unwrap_or_else(|_| {
        CString::new("Error capturing output").unwrap()
    }).into_raw()
}

// Free a string created by interpret_j_code
#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        let _ = CString::from_raw(s);
    }
}