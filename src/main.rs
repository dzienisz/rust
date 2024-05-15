use eframe::{egui, epi};
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum TaskStatus {
    Pending,
    Completed,
}

#[derive(Debug)]
struct Task {
    description: String,
    completed: TaskStatus,
}

type DisplayTasks = Vec<String>;

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: TaskStatus::Pending,
        }
    }

    fn toggle_complete(&mut self) {
        self.completed = match self.completed {
            TaskStatus::Pending => TaskStatus::Completed,
            TaskStatus::Completed => TaskStatus::Pending,
        };
    }
}

struct TaskManager {
    tasks: VecDeque<Task>,
    new_task: String,
    filter: TaskFilter,
}

enum TaskFilter {
    All,
    Completed,
    Incomplete,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: VecDeque::new(),
            new_task: String::new(),
            filter: TaskFilter::All,
        }
    }

    fn add_task(&mut self) {
        let task = Task::new(self.new_task.clone());
        self.tasks.push_back(task);
        self.new_task.clear();
    }

    fn toggle_task_complete(&mut self, index: usize) -> Result<(), String> {
        if let Some(task) = self.tasks.get_mut(index) {
            task.toggle_complete();
            Ok(())
        } else {
            Err("Task not found.".to_string())
        }
    }

    fn remove_task(&mut self, index: usize) -> Result<(), String> {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err("Task not found.".to_string())
        }
    }

    fn display_tasks(&self) -> DisplayTasks {
        self.tasks
            .iter()
            .enumerate()
            .filter(|(_, task)| match self.filter {
                TaskFilter::All => true,
                TaskFilter::Completed => task.completed == TaskStatus::Completed,
                TaskFilter::Incomplete => task.completed == TaskStatus::Pending,
            })
            .map(|(i, task)| format!("{}: {} [{}]", i, task.description, if task.completed == TaskStatus::Completed { "x" } else { " " }))
            .collect()
    }
}

impl epi::App for TaskManager {
    fn name(&self) -> &str {
        "Task Manager"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Task Manager");

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_task);
                if ui.button("Add Task").clicked() {
                    self.add_task();
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Show All").clicked() {
                    self.filter = TaskFilter::All;
                }
                if ui.button("Show Completed").clicked() {
                    self.filter = TaskFilter::Completed;
                }
                if ui.button("Show Incomplete").clicked() {
                    self.filter = TaskFilter::Incomplete;
                }
            });

            ui.separator();

            ui.label("Tasks:");
            for (i, task) in self.display_tasks().iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(task);
                    if ui.button("Toggle Complete").clicked() {
                        if let Err(err) = self.toggle_task_complete(i) {
                            eprintln!("{}", err); // Basic error handling
                        }
                    }
                    if ui.button("Remove").clicked() {
                        if let Err(err) = self.remove_task(i) {
                            eprintln!("{}", err); // Basic error handling
                        }
                    }
                });
            }
        });
    }
}

fn main() {
    let app = TaskManager::new();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
