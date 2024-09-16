use chrono::NaiveDateTime;
use std::env;
use std::fs::File;
use std::io;
use std::path::Path;
use csv::Writer;

fn convert_to_24hr_format(date_time_str: &str) -> Result<String, chrono::format::ParseError> {
    // Separate date and time
    let parts: Vec<&str> = date_time_str.split(',').collect();
    let date_part = parts[0].trim();
    let time_part = parts[1].trim();

    // Parse the 12-hour time and convert to 24-hour time
    let parsed_time = NaiveDateTime::parse_from_str(
        &format!("{} {}", date_part, time_part),
        "%m/%d/%y %I:%M:%S %p"
    )?;
    
    // Return as formatted string
    Ok(parsed_time.format("%m/%d/%y %H:%M:%S").to_string())
}

fn format_name_field(name: &str) -> String {
    // Remove the prefix "Event-<ID>" and the word "Archived"
    let name = name.replace("Archived", "").trim().to_string();
    let name = name.splitn(3, '-').last().unwrap_or(&name);

    // Replace hyphens with spaces and capitalize each word
    let formatted_name = name.replace('-', " ");
    formatted_name
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn generate_video_path(id: &str) -> String {
    format!("videos/{}-video.webm", id)  // Corrected to include -video before .webm
}

fn main() -> io::Result<()> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: zm_clean_export <input_file> <output_file>");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    // Open the input file
    let input_path = Path::new(input_file);
    let input_file = File::open(input_path)?;

    // Create the output file
    let output_path = Path::new(output_file);
    let mut writer = Writer::from_path(output_path)?;

    // Write the metadata header
    writer.write_record(&["Datetime", "Description", "Event ID", "Monitor", "Video File"])?;

    // Process the CSV file
    let reader = csv::Reader::from_reader(input_file);
    for result in reader.into_records() {
        let record = result?;
        
        // Extract and format the fields
        let id = &record[0];
        let name = format_name_field(&record[1]);
        let start_time = convert_to_24hr_format(&record[4].replace("CDT", "").trim()).unwrap_or_else(|_| record[4].replace("CDT", "").trim().to_string());
        let video_path = generate_video_path(id);

        // Write the cleaned record to the output file
        writer.write_record(&[
            &start_time,
            &name,
            id,
            &record[2], // Monitor (kept as-is)
            &video_path,
        ])?;
    }

    writer.flush()?;
    println!("Cleaning complete. The cleaned data has been saved to '{}'.", output_file);

    Ok(())
}

