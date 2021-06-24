// With unsfafe you can..
// Dereference a raw pointer
// Call an unsafe function or method
// Access or modify a mutable static variable
// Implement an unsafe trait
// Access fields of unions

use std::slice;

fn main() {
    derefence_raw_poointer();
    unsafe_function();
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
