mod command;
mod model;
mod task;

use command::{Command, parse_command};
use model::Task;
use task::{add_task, complete_task, list_tasks};

fn main() {
    let mut tasks = Vec::new();

    loop {
        let input = get_user_input();
        let command = parse_command(&input);

        if let Command::Exit = command {
            break;
        }

        handle_command(command, &mut tasks);
    }
}

fn get_user_input() -> String {
    use std::io::{self, Write};

    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("読み取り失敗");
    input.trim().to_string()
}

fn handle_command(command: Command, tasks: &mut Vec<Task>) {
    match command {
        Command::Add(desc) => add_task(desc, tasks),
        Command::List => list_tasks(tasks),
        Command::Done(i) => complete_task(i, tasks),
        Command::Unknown => println!("不明なコマンドです。"),
        Command::Exit => println!("終了します。"),
    }
}
