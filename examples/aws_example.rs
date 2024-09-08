use blockless_sdk::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    //
    // This can be used in with a actual project using the
    // blockless rust sdk, involving the bls command
    //
    // let mut env_buf = [0u8; 4096];
    // let config = Vec::new();
    // match read_env_vars(&mut env_buf) {
    //     Ok(len) => {
    //         let env_vars = String::from_utf8_lossy(&env_buf[..len as usize]);
    //         for var in env_vars.split('\0') {
    //             if !var.is_empty() {
    //                 if let Some((_key, value)) = var.split_once(':') {
    //                     let cleaned_value = value
    //                         .trim()
    //                         .trim_matches(|c| c == '"' || c == '{' || c == '}')
    //                         .trim();

    //                     for part in cleaned_value.split(',') {
    //                         config.push(part.trim().to_string());
    //                     }
    //                 } else {
    //                     println!("{}", var);
    //                 }
    //             }
    //         }
    //     }
    //     Err(e) => eprintln!("Error reading environment variables: {}", e),
    // }

    let config = std::env::var("CONFIG").unwrap();
    let config: Vec<&str> = config.split(',').collect();
    let bucket_name = config[0].to_string();
    let access_key = config[1].to_string();
    let secret_key = config[2].to_string();
    let region = config[3].to_string();
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
    let content = b"S3 foo bar!";
    let put_result = bucket
        .clone()
        .put_object("foo.txt", content.clone().as_mut())?;
    println!("Put object result: {}", put_result);

    // Get the object from the bucket
    let get_result = bucket.clone().get_object("foo.txt")?;
    println!(
        "Get object result content: {}",
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
    let object_name = "foo.txt";
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
