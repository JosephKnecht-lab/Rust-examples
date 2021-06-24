// With unsfafe you can..
// Dereference a raw pointer
// Call an unsafe function or method
// Access or modify a mutable static variable
// Implement an unsafe trait
// Access fields of unions

// Using unsafe to take one of the five actions (superpowers) just discussed isn’t wrong or even frowned upon.
// But it is trickier to get unsafe code correct because the compiler can’t help uphold memory safety. 

use std::slice;

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}


fn main() {
    derefence_raw_poointer();
    unsafe_function();

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    call_from_c();

    println!("name is: {}", HELLO_WORLD);

    modify_mutable_static_var();

}

fn derefence_raw_poointer(){
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // println!("r1 is: {}", *r1);  //cant dereference r1 here

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
 
    let address = 0x012345usize;  //point to exact location in memory
    let r = address as *mut i32;

    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

}

fn unsafe_function(){
    unsafe fn dangerous() {}

    // dangerous(); can't call unsafe function without unsafe block
    
    unsafe {
        dangerous();
    }

}

// calling external function in C language
extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn modify_mutable_static_var(){
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
