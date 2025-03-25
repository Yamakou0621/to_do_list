use std::io;
#[derive(Debug)]
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
        println!("タスクを追加しました。");
    } else if input == "list" {
        if tasks.is_empty() {
            println!("タスクが存在しません。");
        } else {
            for (i, task) in tasks.iter().enumerate() {
                let status = if task.completed { "[x]" } else { "[ ]" };
                println!("{} {} {}", i + 1, status, task.description);
            }
        }
    } else if input.starts_with("done ") {
        let index_str = input.trim_start_matches("done ").trim();
        if let Ok(index) = index_str.parse::<usize>() {
            let i = index - 1;
            if let Some(task) = tasks.get_mut(i) {
                task.completed = true;
                println!("タスク{} を完了にしました。", index);
            } else {
                println!("その番号のタスクは存在しません。");
            }
        } else {
            println!("番号を正しく入力してください。例: done 1");
        }
    } else {
        println!("不明なコマンドです");
    }
}
