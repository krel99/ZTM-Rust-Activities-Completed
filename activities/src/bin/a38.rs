// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    let handle_hello = std::thread::spawn(msg_hello);
    let handle_thread = std::thread::spawn(msg_thread);
    let handle_excited = std::thread::spawn(msg_excited);

    let hello = handle_hello.join().expect("failed to join hello thread");
    let thread = handle_thread.join().expect("failed to join thread thread");
    let excited = handle_excited
        .join()
        .expect("failed to join excited thread");

    println!("{}{}{}", hello, thread, excited);
}
