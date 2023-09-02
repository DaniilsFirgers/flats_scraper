use crate::scrape_flats::districts::DISTRICTS;
use crate::scrape_flats::structs::Flat;
use scraper::{Html, Selector};
use std::io::{BufWriter, Write};
use std::{
    fs::{self, File},
    io,
};

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

pub fn save_to_local_json_file(
    flat_data: Vec<Flat>,
    district_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create the directory if it doesn't exist
    let path = format!("data/{}.json", district_name);
    fs::create_dir_all("data")?;
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &flat_data)?;
    writer.flush()?;
    Ok(())
}

pub fn parse_string_to_int(string_value: String) -> Result<i32, Box<dyn std::error::Error>> {
    let parsed_i32: i32 = string_value.parse()?;

    Ok(parsed_i32)
}

pub fn find_first_main_page_element(document: &Html, main_pages_selector: &Selector) -> bool {
    document.select(main_pages_selector).next().is_none()
}
