use super::districts::DISTRICTS;
use std::io;

pub fn read_user_input() -> String {
    let mut district = String::new();
    loop {
        district.clear();
        println!("Enter district name from the list {:#?}: ", DISTRICTS);

        match io::stdin().read_line(&mut district) {
            Ok(_) => {
                let lowercase_district = district.to_lowercase().trim().to_string();
                if DISTRICTS.contains(&lowercase_district.as_str()) {
                    return lowercase_district;
                } else {
                    println!("Invalid district. Please try again.");
                }
            }
            Err(err) => {
                eprintln!("Failed to read input: {}. Please try again!", err);
            }
        }
    }
}
