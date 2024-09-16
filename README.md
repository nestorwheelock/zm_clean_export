
# zm_clean_export

**zm_clean_export** is a Rust-based tool designed to clean and format exported CSV data from ZoneMinder. It processes the CSV file, formats the timestamps into 24-hour format, cleans up event names, and generates paths for video files associated with each event.

## Features

- **Timestamp Formatting**: Converts timestamps from 12-hour to 24-hour format.
- **Event Name Cleanup**: Removes prefixes (e.g., `Event-<ID>`) and the word "Archived" from event names, then formats and capitalizes each word.
- **Video Path Generation**: Creates paths for video files corresponding to each event in the format `videos/<event_id>-video.webm`.
- **CSV Output**: Outputs the cleaned data into a new CSV file with the following columns: `Datetime`, `Description`, `Event ID`, `Monitor`, `Video File`.

## Usage

To use **zm_clean_export**, you need to specify an input CSV file and an output CSV file. The tool will read the input file, clean the data, and save it to the specified output file.

### Command

```bash
./target/release/zm_clean_export <input_file> <output_file>
```

### Example

```bash
./target/release/zm_clean_export events.csv cleaned_events.csv
```

This command will:

1. Read `events.csv` as the input.
2. Clean the data, format timestamps, and generate video paths.
3. Save the cleaned data to `cleaned_events.csv`.

### Input Format

- The program expects the input CSV file to contain records with at least the following fields:
  - **Event ID** (first column)
  - **Event Name** (second column)
  - **Monitor** (third column)
  - **Start Time** (fifth column, in the format `%m/%d/%y %I:%M:%S %p`)

### Output Format

The cleaned CSV will contain the following columns:
1. `Datetime` (in 24-hour format)
2. `Description` (cleaned and formatted event name)
3. `Event ID`
4. `Monitor`
5. `Video File` (generated as `videos/<event_id>-video.webm`)

## License

This project is licensed under the GPLv3 License. See the [LICENSE](LICENSE) file for more details.
