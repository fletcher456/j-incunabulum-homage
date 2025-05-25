use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use libc::{malloc, strlen};

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

// Macros translated to functions or inline code

// Helper function for DO macro: executes the closure n times with index i
#[inline]
unsafe fn do_loop<F>(n: I, mut func: F) where F: FnMut(I) {
    let mut i: I = 0;
    let _n = n;
    while i < _n {
        func(i);
        i += 1;
    }
}

// Memory allocation - equivalent to ma(n) in the fragment
#[no_mangle]
pub unsafe extern "C" fn ma(n: I) -> *mut I {
    malloc((n * 4) as usize) as *mut I
}

// Memory copy - equivalent to mv(d,s,n) in the fragment
#[no_mangle]
pub unsafe extern "C" fn mv(d: *mut I, s: *mut I, n: I) {
    do_loop(n, |i| {
        *d.offset(i as isize) = *s.offset(i as isize);
    });
}

// Calculate total size - equivalent to tr(r,d) in the fragment
#[no_mangle]
pub unsafe extern "C" fn tr(r: I, d: *mut I) -> I {
    let mut z: I = 1;
    do_loop(r, |i| {
        z = z * *d.offset(i as isize);
    });
    z
}

// Create array - equivalent to ga(t,r,d) in the fragment
#[no_mangle]
pub unsafe extern "C" fn ga(t: I, r: I, d: *mut I) -> A {
    let z = ma(5 + tr(r, d)) as A;
    (*z).t = t;
    (*z).r = r;
    mv((*z).d.as_mut_ptr(), d, r);
    z
}

// Iota function - equivalent to iota(w) in the fragment
#[no_mangle]
pub unsafe extern "C" fn iota(w: A) -> A {
    let mut n = *(*w).p.as_ptr();
    let z = ga(0, 1, &mut n);
    do_loop(n, |i| {
        *(*z).p.as_mut_ptr().offset(i as isize) = i;
    });
    z
}

// Plus function - equivalent to plus(a,w) in the fragment
#[no_mangle]
pub unsafe extern "C" fn plus(a_ptr: A, w: A) -> A {
    let r = (*w).r;
    let d = (*w).d.as_mut_ptr();
    let n = tr(r, d);
    let z = ga(0, r, d);
    do_loop(n, |i| {
        *(*z).p.as_mut_ptr().offset(i as isize) = 
            *(*a_ptr).p.as_mut_ptr().offset(i as isize) + 
            *(*w).p.as_mut_ptr().offset(i as isize);
    });
    z
}

// From function - equivalent to from(a,w) in the fragment
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

// Box function - equivalent to box(w) in the fragment
#[no_mangle]
pub unsafe extern "C" fn box_func(w: A) -> A {
    let z = ga(1, 0, std::ptr::null_mut());
    *(*z).p.as_mut_ptr() = w as I;
    z
}

// Cat function - equivalent to cat(a,w) in the fragment
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

// Find function - empty placeholder as in the fragment
#[no_mangle]
pub unsafe extern "C" fn find(_a: A, _w: A) -> A {
    std::ptr::null_mut()
}

// Reshape function - equivalent to rsh(a,w) in the fragment
#[no_mangle]
pub unsafe extern "C" fn rsh(a_ptr: A, w: A) -> A {
    let r = if (*a_ptr).r > 0 { *(*a_ptr).d.as_ptr() } else { 1 };
    let n = tr(r, (*a_ptr).p.as_mut_ptr());
    let wn = tr((*w).r, (*w).d.as_mut_ptr());
    let z = ga((*w).t, r, (*a_ptr).p.as_mut_ptr());
    let wn_adjusted = if n > wn { wn } else { n };
    mv((*z).p.as_mut_ptr(), (*w).p.as_mut_ptr(), wn_adjusted);
    let remaining = n - wn_adjusted;
    if remaining > 0 {
        mv(
            (*z).p.as_mut_ptr().offset(wn_adjusted as isize),
            (*z).p.as_mut_ptr(),
            remaining
        );
    }
    z
}

// Shape function - equivalent to sha(w) in the fragment
#[no_mangle]
pub unsafe extern "C" fn sha(w: A) -> A {
    let z = ga(0, 1, &mut (*w).r);
    mv((*z).p.as_mut_ptr(), (*w).d.as_mut_ptr(), (*w).r);
    z
}

// Identity function - equivalent to id(w) in the fragment
#[no_mangle]
pub unsafe extern "C" fn id(w: A) -> A {
    w
}

// Size function - equivalent to size(w) in the fragment
#[no_mangle]
pub unsafe extern "C" fn size(w: A) -> A {
    let z = ga(0, 0, std::ptr::null_mut());
    *(*z).p.as_mut_ptr() = if (*w).r > 0 { *(*w).d.as_ptr() } else { 1 };
    z
}

// Print integer - equivalent to pi(i) in the fragment
#[no_mangle]
pub unsafe extern "C" fn pi(i: I) {
    print!("{} ", i);
}

// Print newline - equivalent to nl() in the fragment
#[no_mangle]
pub unsafe extern "C" fn nl() {
    println!();
}

// Print array - equivalent to pr(w) in the fragment
#[no_mangle]
pub unsafe extern "C" fn pr(w: A) {
    let r = (*w).r;
    let d = (*w).d.as_mut_ptr();
    let n = tr(r, d);
    do_loop(r, |i| {
        pi(*d.offset(i as isize));
    });
    nl();
    
    if (*w).t != 0 {
        do_loop(n, |i| {
            print!("< ");
            pr(*(*w).p.as_mut_ptr().offset(i as isize) as A);
        });
    } else {
        do_loop(n, |i| {
            pi(*(*w).p.as_mut_ptr().offset(i as isize));
        });
    }
    nl();
}

// Verb table - equivalent to vt[] in the fragment
static mut VT: [C; 7] = [b'+' as C, b'{' as C, b'~' as C, b'<' as C, b'#' as C, b',' as C, 0];

// Function pointers arrays - we need to define these properly
// We'll populate these in the initialization code
static mut VD: [Option<unsafe extern "C" fn(A, A) -> A>; 7] = [None; 7];
static mut VM: [Option<unsafe extern "C" fn(A) -> A>; 7] = [None; 7];

// Symbol table - equivalent to st[] in the fragment
static mut ST: [I; 26] = [0; 26];

// Check if character is a variable name - equivalent to qp(a) in the fragment
#[no_mangle]
pub unsafe extern "C" fn qp(a: I) -> I {
    if a >= 'a' as I && a <= 'z' as I { 1 } else { 0 }
}

// Check if character is a verb - equivalent to qv(a) in the fragment
#[no_mangle]
pub unsafe extern "C" fn qv(a: I) -> I {
    if a < 'a' as I { 1 } else { 0 }
}

// Execute expression - equivalent to ex(e) in the fragment
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
            return std::ptr::null_mut();
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
        return std::ptr::null_mut();
    } else if *e.offset(1) != 0 {
        let idx = *e.offset(1) as usize;
        if idx < VD.len() && VD[idx].is_some() {
            let vd_fn = VD[idx].unwrap();
            return vd_fn(a as A, ex(e.offset(2)));
        }
        return std::ptr::null_mut();
    } else {
        return a as A;
    }
}

// Parse noun - equivalent to noun(c) in the fragment
#[no_mangle]
pub unsafe extern "C" fn noun(c: C) -> A {
    if c < '0' as C || c > '9' as C {
        return std::ptr::null_mut();
    }
    
    let z = ga(0, 0, std::ptr::null_mut());
    *(*z).p.as_mut_ptr() = c as I - '0' as I;
    z
}

// Parse verb - equivalent to verb(c) in the fragment
#[no_mangle]
pub unsafe extern "C" fn verb(c: C) -> I {
    let mut i: I = 0;
    while VT[i as usize] != 0 {
        if VT[i as usize] == c {
            return i + 1;
        }
        i += 1;
    }
    0
}

// Parse words - equivalent to wd(s) in the fragment
#[no_mangle]
pub unsafe extern "C" fn wd(s: *const C) -> *mut I {
    let n = strlen(s as *const i8) as I;
    let e = ma(n + 1);
    
    do_loop(n, |i| {
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
    });
    
    *e.offset(n as isize) = 0;
    e
}

// Initialize the function pointer arrays
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

// Custom print implementation that writes to a string buffer
pub struct StringPrinter {
    buffer: String,
}

impl StringPrinter {
    pub fn new() -> Self {
        StringPrinter {
            buffer: String::new(),
        }
    }
    
    pub fn print(&mut self, s: &str) {
        self.buffer.push_str(s);
    }
    
    pub fn println(&mut self, s: &str) {
        self.buffer.push_str(s);
        self.buffer.push('\n');
    }
    
    pub fn get_buffer(&self) -> &str {
        &self.buffer
    }
}

// Override print functions to use our custom printer
#[no_mangle]
pub unsafe extern "C" fn pi_custom(i: I, printer: &mut StringPrinter) {
    printer.print(&format!("{} ", i));
}

#[no_mangle]
pub unsafe extern "C" fn nl_custom(printer: &mut StringPrinter) {
    printer.println("");
}

#[no_mangle]
pub unsafe extern "C" fn pr_custom(w: A, printer: &mut StringPrinter) {
    let r = (*w).r;
    let d = (*w).d.as_mut_ptr();
    let n = tr(r, d);
    
    do_loop(r, |i| {
        pi_custom(*d.offset(i as isize), printer);
    });
    nl_custom(printer);
    
    if (*w).t != 0 {
        do_loop(n, |i| {
            printer.print("< ");
            pr_custom(*(*w).p.as_mut_ptr().offset(i as isize) as A, printer);
        });
    } else {
        do_loop(n, |i| {
            pi_custom(*(*w).p.as_mut_ptr().offset(i as isize), printer);
        });
    }
    nl_custom(printer);
}

// Exported function to interpret J code
#[no_mangle]
pub unsafe extern "C" fn interpret_j_code(input: *const c_char) -> *mut c_char {
    // Initialize function tables if not already done
    init_function_tables();
    
    // Create our custom printer
    let mut printer = StringPrinter::new();
    
    // Convert C string to Rust
    let c_str = CStr::from_ptr(input);
    let rust_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return CString::new("Error: Invalid input string").unwrap().into_raw(),
    };
    
    // Convert Rust string to char array for C
    let c_input = rust_str.as_ptr() as *const C;
    
    // Execute J code
    let result = ex(wd(c_input));
    if !result.is_null() {
        pr_custom(result, &mut printer);
    } else {
        printer.println("Error evaluating J expression");
    }
    
    // Convert captured output to C string
    match CString::new(printer.get_buffer()) {
        Ok(s) => s.into_raw(),
        Err(_) => CString::new("Error capturing output").unwrap().into_raw(),
    }
}

// Free a string created by interpret_j_code
#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        let _ = CString::from_raw(s);
    }
}