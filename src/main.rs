use std::io;

struct Task {
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks = Vec::new();

    loop {
        let input = get_user_input();

        if input == "exit" {
            break;
        }

        handle_command(&input, &mut tasks);
    }
}

// ユーザー入力の取得
fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("読み取り失敗");
    input.trim().to_string()
}

// コマンド処理
fn handle_command(input: &str, tasks: &mut Vec<Task>) {
    if input.starts_with("add ") {
        let desc = input.trim_start_matches("add ").to_string();
        tasks.push(Task {
            description: desc,
            completed: false,
        });
        println!("タスクを追加しました");
    } else if input == "list" {
        for (i, task) in tasks.iter().enumerate() {
            let status = if task.completed { "[x]" } else { "[ ]" };
            println!("{} {} {}", i + 1, status, task.description);
        }
    } else {
        println!("不明なコマンドです");
    }
}
