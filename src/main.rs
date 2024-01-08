mod helloworld;
mod random_num;
mod ownership;
mod structure;
mod panic;
mod enums;
mod collections;
mod error_handling;
use helloworld::hello;


fn main() {

    // This is how u import a function from different file. mod filename(prolly)
    // helloworld::main();
    random_num::random_num();
    // ownership::ownership_fn();
    // structure::main();
    // panic::main();
    // enums::main();
    // hello::hello();
    // collections::main();
    error_handling::main();
}
