use eframe::{egui, epi};
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

    fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.complete();
        } else {
            eprintln!("Task not found.");
        }
    }

    fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
        } else {
            eprintln!("Task not found.");
        }
    }

    fn filtered_tasks(&self) -> Vec<String> {
        self.tasks.iter()
            .enumerate()
            .filter(|(_, task)| match self.filter {
                TaskFilter::All => true,
                TaskFilter::Completed => task.completed,
                TaskFilter::Incomplete => !task.completed,
            })
            .map(|(i, task)| format!("{}: {} [{}]", i, task.description, if task.completed { "x" } else { " " }))
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
            for (i, task) in self.filtered_tasks().iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(task);
                    if ui.button("Complete").clicked() {
                        self.complete_task(i);
                    }
                    if ui.button("Remove").clicked() {
                        self.remove_task(i);
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
