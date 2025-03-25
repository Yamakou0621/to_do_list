#[derive(Debug)]
pub enum Command<'a> {
    Add(&'a str),
    List,
    Done(usize),
    Exit,
    Unknown,
}

pub fn parse_command(input: &str) -> Command {
    let input = input.trim();
    let (cmd, arg) = input
        .split_once(' ')
        .map(|(c, a)| (c, a.trim()))
        .unwrap_or((input, ""));

    match cmd {
        "add" => Command::Add(arg),
        "list" => Command::List,
        "done" => {
            if let Ok(i) = arg.parse::<usize>() {
                Command::Done(i)
            } else {
                Command::Unknown
            }
        }
        "exit" => Command::Exit,
        _ => Command::Unknown,
    }
}
