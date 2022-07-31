/**
 * In this module we will go through few examples and explain concurrency in rust lang.
 * Thread example. (line 10)
 * Join Handles example. (line 23)
 */
pub mod module {
    use std::thread;
    use std::time::Duration;
    pub fn concurrency() {
        // Thread example.
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        // Code executed by the main thread.
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        // Join Handles example.
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        handle.join().unwrap();
    }
}