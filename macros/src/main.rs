// Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
// Attribute-like macros that define custom attributes usable on any item
// Function-like macros that look like function calls but operate on the tokens specified as their argument
use proc_macro;


#[macro_export]
macro_rules! vec {    // vec! macro definition
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// #[some_attribute]        example procedural macro.. Taje 
// pub fn some_name(input: TokenStream) -> TokenStream {
// }


fn main() {
    let v: Vec<u32> = vec![1, 2, 3];  // simple vec! macro


}


