mod common;
mod days;
use days::DAYS;

fn main() {
    match get_args() {
        Some((day, part)) => {
            let input = std::io::stdin()
                .lines()
                .into_iter()
                .map(|line| line.unwrap())
                .collect::<Vec<String>>()
                .join("\n");
            if let Some(part) = part {
                run_day(day, part, &input);
            } else {
                run_day(day, 1, &input);
                run_day(day, 2, &input);
            }
        }
        _ => usage(),
    }
}

fn run_day(day: usize, part: usize, input: &str) {
    let day = DAYS
        .get(day - 1)
        .unwrap_or_else(|| panic!("{} is not a valid day", day));
    let func = day
        .get(part - 1)
        .unwrap_or_else(|| panic!("{} is not a valid part", part));
    func(input);
}

fn get_args() -> Option<(usize, Option<usize>)> {
    let args = std::env::args().collect::<Vec<String>>();
    let day = args.get(1)?.parse().expect("'day' must be a number");
    let part = args
        .get(2)
        .map(|arg| arg.parse().expect("'part' must be a number"));
    Some((day, part))
}

fn usage() {
    let binary_name = std::env::current_exe()
        .ok()
        .and_then(|path| {
            let file_name = path.file_name()?;
            let name_str = file_name.to_str()?;
            Some(name_str.to_string())
        })
        .unwrap_or_else(|| String::from("<binary>"));
    println!("USAGE: {} <day> [part]", binary_name);
}
