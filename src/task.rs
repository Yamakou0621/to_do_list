use crate::model::Task;

pub fn add_task(description: &str, tasks: &mut Vec<Task>) {
    if description.trim().is_empty() {
        println!("タスクの内容を入力してください。");
        return;
    }
    tasks.push(Task {
        description: description.to_string(),
        completed: false,
    });
    println!("タスクを追加しました: {}", description);
}

pub fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("タスクが存在しません。");
    } else {
        for (i, task) in tasks.iter().enumerate() {
            let status = if task.completed { "[x]" } else { "[ ]" };
            println!("{} {} {}", i + 1, status, task.description);
        }
    }
}

pub fn complete_task(index: usize, tasks: &mut Vec<Task>) {
    let i = index - 1;
    if let Some(task) = tasks.get_mut(i) {
        task.completed = true;
        println!("タスク {} を完了にしました。", index);
    } else {
        println!("その番号のタスクは存在しません。");
    }
}
