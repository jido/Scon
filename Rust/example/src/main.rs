use serde_scon::to_string_pretty;
use serde_json::{Result, Value};
use std::io::Read;

fn main() -> Result<()> {
    println!("- Waiting for JSON -");
    let mut data: String = "".to_string();
    match std::io::stdin().read_to_string(&mut data) {
        Ok(n) if n > 0 => {
            println!("{} characters", n);
        }
        _ => {
            println!(" * No data, using default JSON");
            data = r#"
            {
              "name": "John Doe",
              "age": 43,
              "phones": [
                "+44 1234567",
                "+44 2345678"
              ]
            }"#.to_owned();
        }
    }

    // Parse the string of JSON data using serde_json.
    match serde_json::from_str::<Value>(&data) {
        Ok(v) => {
            // Print the data in Scon format
            println!("- Converted to Scon -");
            println!("{}", to_string_pretty(&v).unwrap());
        }
        Err(e) => {
            println!("JSON error: {:?}", e);
        }
    }

    Ok(())
}
