use blockless_sdk::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let bucket_name = "".to_string();
    let secret_key = "".to_string();
    let access_key = "".to_string();
    let region = "".to_string();
    let s3_config = AWSS3Configure::new(access_key, secret_key, region);

    // Create a bucket instance
    let bucket = Bucket::new(bucket_name, s3_config);

    // List objects in the bucket
    println!("Listing objects in the bucket:");
    let list_result = bucket.clone().list("")?;
    for response in list_result {
        println!("Bucket: {}", response.name);
        if response.contents.is_empty() {
            println!("Bucket is empty");
        }
    }

    // Put an object in the bucket
    let content = b"Hello, Blockless S3!";
    let put_result = bucket
        .clone()
        .put_object("example.txt", content.clone().as_mut())?;
    println!("Put object result: {}", put_result);

    // Get the object from the bucket
    let get_result = bucket.clone().get_object("example.txt")?;
    println!(
        "Get object result: {}",
        String::from_utf8_lossy(&get_result)
    );

    // List objects in the bucket
    println!("Listing objects in the bucket:");
    let list_result = bucket.clone().list("")?;
    for response in list_result {
        println!("Bucket: {}", response.name);
        if !response.contents.is_empty() {
            for content in response.contents {
                println!("  {}", content);
            }
        }
    }

    // Delete the object from the bucket
    let object_name = "example.txt";
    let _ = bucket
        .clone()
        .delete_object(object_name)
        .is_err_and(|e| match e {
            AWSS3ErrorKind::RequestError => {
                println!("Object {} deleted", object_name);
                true
            }
            _ => false,
        });

    Ok(())
}
