extern crate coroutine;

use coroutine::{spawn, sched};

fn main() {
    // Spawn a new coroutine
    let coro = spawn(move|| {

        println!("Hello in coroutine!");

        // Yield back to it's parent
        sched();

        println!("We are back!!");

        // Spawn a new coroutine
        spawn(move|| {
            println!("Hello inside");
        }).join().ok().expect("Failed to join");

        println!("Good bye");
    });

    coro.resume().ok().expect("Failed to resume");

    println!("We are here!");

    // Resume the coroutine
    coro.resume().ok().expect("Failed to resume");

    println!("Back to main.");
}
