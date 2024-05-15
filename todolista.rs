use std::io;
use std::collections::VecDeque;

#[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

struct TaskManager {
    tasks: VecDeque<Task>,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: VecDeque::new(),
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Task::new(description);
        self.tasks.push_back(task);
    }

    fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.complete();
        } else {
            println!("Task not found.");
        }
    }

    fn list_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            println!("{}: {} [{}]", i, task.description, if task.completed { "x" } else { " " });
        }
    }

    fn chatbot(&mut self, input: &str) {
        if input.contains("add") {
            let parts: Vec<&str> = input.splitn(2, "add ").collect();
            if parts.len() > 1 {
                self.add_task(parts[1].trim().to_string());
                println!("Task added: {}", parts[1].trim());
            } else {
                println!("Please specify a task description.");
            }
        } else if input.contains("complete") {
            let parts: Vec<&str> = input.splitn(2, "complete ").collect();
            if parts.len() > 1 {
                let index: usize = match parts[1].trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid task index.");
                        return;
                    },
                };
                self.complete_task(index);
                println!("Task completed: {}", index);
            } else {
                println!("Please specify a task index.");
            }
        } else if input.contains("list") {
            self.list_tasks();
        } else {
            println!("I didn't understand that command.");
        }
    }
}

fn main() {
    let mut manager = TaskManager::new();
    println!("Welcome to the Task Manager Chatbot!");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input == "exit" {
            break;
        }
        manager.chatbot(input);
    }
}
