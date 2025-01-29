use slug::slugify;
use std::env;
use std::io::{self, BufRead};

fn main() {
    fn slugify_fr(s: String) -> String {
        let slug_len = s.len();
        let spaces = " ".repeat(slug_len);
        let underscores = "_".repeat(slug_len);
        // Art by Dirk-Lueder Kreie but i have removed the shell
        // https://www.asciiart.eu/animals/insects/snails
        format!(
            r#"
            {}      oo
        ,___{}______||
 __,..="^  .{} , "  , \
<.__________{}________/
"#,
            spaces, underscores, s, underscores
        )
    }
    fn cheese(s: String) -> String {
        let cheese_len = s.split_whitespace().count();
        "cheese ".repeat(cheese_len)
    }

    let help_text = r#"Valid arguments are the following:
  lowercase  - Convert entire text to lowercase
  uppercase  - Convert entire text to uppercase
  no-spaces  - Remove all spaces from text
  slugify    - Convert text into URL-friendly slug;
  cheese     - Convert all the words into cheese;
  slugify-fr - Convert text into a friendly slug for real"#;

    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Error: no arguments provided!");
        println!("{}", help_text);
        return;
    }

    if args.len() > 1 {
        println!("Error: more than one argument provided!");
        println!("{}", help_text);
        return;
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let result = match args[0].as_str() {
            "lowercase" => line.unwrap().to_lowercase(),
            "uppercase" => line.unwrap().to_uppercase(),
            "no-spaces" => line.unwrap().replace(" ", ""),
            "slugify" => slugify(line.unwrap()),
            "cheese" => cheese(line.unwrap()),
            "slugify-fr" => slugify_fr(line.unwrap()),
            invalid => {
                println!("Error: Invalid transformation: {}", invalid);
                println!("{}", help_text);
                return;
            }
        };
        println!("{}", result);
    }
}
