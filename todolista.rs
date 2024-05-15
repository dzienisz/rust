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
}

fn main() {
    let mut manager = TaskManager::new();
    loop {
        println!("1. Add task");
        println!("2. Complete task");
        println!("3. List tasks");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                let mut description = String::new();
                println!("Enter task description:");
                io::stdin().read_line(&mut description).expect("Failed to read line");
                manager.add_task(description.trim().to_string());
            }
            2 => {
                let mut index = String::new();
                println!("Enter task index to complete:");
                io::stdin().read_line(&mut index).expect("Failed to read line");
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                manager.complete_task(index);
            }
            3 => manager.list_tasks(),
            4 => break,
            _ => continue,
        }
    }
}
