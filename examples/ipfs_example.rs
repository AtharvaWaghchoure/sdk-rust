use blockless_sdk::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a directory
    let dir_path = "/foo";
    match ipfs_create_dir(dir_path, true) {
        Ok(true) => println!("Directory '{}' created successfully", dir_path),
        Ok(false) => println!("Failed to create directory '{}'", dir_path),
        Err(e) => println!("Error creating directory: {:?}", e),
    }

    // Write a file
    let file_path = "/foo/bar.txt";
    let content = b"foo bar";
    let write_opts = FileWriteOptions {
        file: file_path.to_string(),
        offset: 0,
        create: true,
        parents: true,
        truncate: false,
    };
    match ipfs_file_write(&write_opts, content.clone().as_mut()) {
        Ok(true) => println!("File '{}' written successfully", file_path),
        Ok(false) => println!("Failed to write file '{}'", file_path),
        Err(e) => println!("Error writing file: {:?}", e),
    }

    // Update a file
    let file_path = "/foo/bar.txt";
    let content = b"\nnew foo bar";
    let edit_opts = FileWriteOptions {
        file: file_path.to_string(),
        offset: read_entire_file(file_path).unwrap().len() as i64 + 1,
        create: false,
        parents: false,
        truncate: false,
    };
    match ipfs_file_write(&edit_opts, content.clone().as_mut()) {
        Ok(true) => println!("File {} edited successfully", file_path),
        Ok(false) => println!("Failed to write file '{}'", file_path),
        Err(e) => println!("Error warning file: {:?}", e),
    }

    // Read the file
    let mut buffer = vec![0u8; 100];
    match ipfs_file_read(file_path, 0, &mut buffer) {
        Ok(bytes_read) => {
            println!("Read {} bytes from '{}'", bytes_read, file_path);
            println!(
                "Content: {}",
                String::from_utf8_lossy(&buffer[..bytes_read])
            );
        }
        Err(e) => println!("Error reading file: {:?}", e),
    }

    // List files in the directory
    match ipfs_file_list(Some(dir_path)) {
        Ok(files) => {
            println!("Files in '{}':", dir_path);
            for file in files {
                println!("1. {}", file);
            }
        }
        Err(e) => println!("Error listing files: {:?}", e),
    }

    // Get file stats
    match ipfs_file_stat(file_path) {
        Ok(stats) => {
            println!("File stats for '{}':", file_path);
            println!("stats: {}\n", stats);
        }
        Err(e) => println!("Error getting file stats: {:?}", e),
    }

    // Remove the original file
    match ipfs_file_remove(file_path, false, false) {
        Ok(true) => println!("File '{}' removed successfully", file_path),
        Ok(false) => println!("Failed to remove file '{}'", file_path),
        Err(e) => println!("Error removing file: {:?}", e),
    }

    Ok(())
}
