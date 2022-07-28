/**
 * In this module we will go through few examples and explain file input and output in rust lang.
 * Create a file example. (line 17)
 * Write to a file example. (line 20)
 * Read from a file example. (line 25)
 * Append data to a file example. (line 31)
 * Copy a file example. (line 37)
 * Delete a file example. (line 54)
 */
pub mod module {
    use std::fs::File;
    use std::io::Write;
    use std::io::Read;
    use std::fs;
    use std::fs::OpenOptions;
    pub fn file_input_output() {
        // Create a file.
        let mut file = File::create("data.txt").expect("create failed");

        // Write to a file.
        file.write_all("Hello World".as_bytes()).expect("write failed");
        file.write_all("\nLearn rust".as_bytes()).expect("write failed");
        println!("data written to file" );

        // Read from file.
        let mut file = File::open("data.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);

        // Append data to a file
        let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
        file.write_all("Hello user".as_bytes()).expect("write failed");
        file.write_all("\nRust help".as_bytes()).expect("write failed");
        println!("file append success");

        // Copy file.
        let mut command_line: std::env::Args = std::env::args();
        command_line.next().unwrap();
        // skip the executable file name
        // accept the source file
        let source = command_line.next().unwrap();
        // accept the destination file
        let destination = command_line.next().unwrap();
        let mut file_in = File::open(source).unwrap();
        let mut file_out = File::create(destination).unwrap();
        let mut buffer = [0u8; 4096];
        loop {
            let bytes = file_in.read(&mut buffer).unwrap();
            file_out.write(&buffer[..bytes]).unwrap();
            if bytes < buffer.len() { break; }
        }

        // Delete a file
        fs::remove_file("data.txt").expect("could not remove file");
        println!("file is removed");
    }
}