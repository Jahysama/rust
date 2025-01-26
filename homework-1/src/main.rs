use std::{io, thread, time::Duration};

fn main() {
    let text = r#"
 _          _ _         _          _     __  \\//
| |__   ___| | | ___   | |   _   _| | __/_/_ _\/ 
| '_ \ / _ \ | |/ _ \  | |  | | | | |/ / _` / __|
| | | |  __/ | | (_) | | |__| |_| |   < (_| \__ \
|_| |_|\___|_|_|\___/  |_____\__,_|_|\_\__,_|___/
"#;
    for c in text.chars() {
        print!("{}", c);
        let _ = io::Write::flush(&mut io::stdout());
        thread::sleep(Duration::from_millis(10));
    }
}
