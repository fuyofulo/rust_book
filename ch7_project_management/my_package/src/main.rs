use my_package::greet;
fn main() {
    greet();
}

// this is the main.rs file and it can be referred as a crate. it is converted into an executable
// since this is where the execution starts. since we are using the greet function which is defined
// in the lob.rs fiel, we need to specify the path on top of the file. the functions used in this
// file maybe defined in other files, so the compiler looks at the path and goes to those files to
// make them executable as well. 
