use csv::Reader;
use nu_ansi_term::{Color, Style};
use nu_table::{NuTable, TableTheme, TextStyle};
use slug::slugify;
use std::env;
use std::error::Error;
use std::io::{self, BufRead};

const HELP_TEXT: &str = r#"Valid arguments are the following:
  lowercase  - Convert entire text to lowercase
  uppercase  - Convert entire text to uppercase
  no-spaces  - Remove all spaces from text
  slugify    - Convert text into URL-friendly slug
  cheese     - Convert all the words into cheese
  slugify-fr - Convert text into a friendly slug for real
  csv        - Parse input as CSV and display as table"#;

fn slugify_fr(s: String) -> Result<String, Box<dyn Error>> {
    let slug_len = s.len();
    let spaces = " ".repeat(slug_len);
    let underscores = "_".repeat(slug_len);
    // Art by Dirk-Lueder Kreie but i have removed the shell
    // https://www.asciiart.eu/animals/insects/snails
    Ok(format!(
        r#"
            {}      oo
        ,___{}______||
 __,..="^  .{} , "  , \
<.__________{}________/
"#,
        spaces, underscores, s, underscores
    ))
}

fn cheese(s: String) -> Result<String, Box<dyn Error>> {
    let cheese_len = s.split_whitespace().count();
    Ok("cheese ".repeat(cheese_len))
}

fn lowercase(s: String) -> Result<String, Box<dyn Error>> {
    Ok(s.to_lowercase())
}

fn uppercase(s: String) -> Result<String, Box<dyn Error>> {
    Ok(s.to_uppercase())
}

fn no_spaces(s: String) -> Result<String, Box<dyn Error>> {
    Ok(s.replace(' ', ""))
}

fn slugify_wrapper(s: String) -> Result<String, Box<dyn Error>> {
    Ok(slugify(&s))
}

fn display_csv() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        let mut line = String::new();
        handle.read_line(&mut line)?;

        if line.trim().is_empty() {
            break;
        }
        input.push_str(&line);
    }

    let record_count = Reader::from_reader(input.as_bytes()).records().count();

    let mut rdr = Reader::from_reader(input.as_bytes());
    let headers = rdr.headers()?.clone();

    let mut table = NuTable::new(record_count + 1, headers.len());

    for (i, header) in headers.iter().enumerate() {
        let styled_header = Color::Blue.bold().paint(header).to_string();
        table.insert((0, i), styled_header);
    }

    let mut row_idx = 1;
    for result in rdr.records() {
        let record = result?;
        for (col_idx, field) in record.iter().enumerate() {
            let styled_field = if row_idx % 2 == 0 {
                Style::new().fg(Color::Fixed(245)).paint(field).to_string()
            } else {
                Style::new().fg(Color::Fixed(252)).paint(field).to_string()
            };
            table.insert((row_idx, col_idx), styled_field);
        }
        row_idx += 1;
    }

    table.set_header_style(TextStyle::basic_center());
    table.set_data_style(TextStyle::basic_left());
    table.set_theme(TableTheme::rounded());
    table.set_structure(false, true, false);

    Ok(table
        .draw(120)
        .unwrap_or_else(|| "Error drawing table".to_string()))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Error: no arguments provided!");
        eprintln!("{}", HELP_TEXT);
        return Ok(());
    }
    if args.len() > 1 {
        eprintln!("Error: more than one argument provided!");
        eprintln!("{}", HELP_TEXT);
        return Ok(());
    }

    eprintln!("Selected operation: {}", args[0]);

    let result = match args[0].as_str() {
        "csv" => display_csv(),
        _ => {
            let stdin = io::stdin();
            let mut output = String::new();
            for line in stdin.lock().lines() {
                let line = match line {
                    Ok(line) => line,
                    Err(e) => {
                        eprintln!("Error reading input: {}", e);
                        continue;
                    }
                };

                let processed = match args[0].as_str() {
                    "lowercase" => lowercase(line),
                    "uppercase" => uppercase(line),
                    "no-spaces" => no_spaces(line),
                    "slugify" => slugify_wrapper(line),
                    "cheese" => cheese(line),
                    "slugify-fr" => slugify_fr(line),
                    invalid => {
                        eprintln!("Error: Invalid transformation: {}", invalid);
                        eprintln!("{}", HELP_TEXT);
                        return Ok(());
                    }
                }?;
                output.push_str(&processed);
                output.push('\n');
            }
            Ok(output)
        }
    };

    match result {
        Ok(output) => print!("{}", output),
        Err(e) => eprintln!("Error in {} operation: {}", args[0], e),
    }

    Ok(())
}
