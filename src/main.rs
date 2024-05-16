use eframe::egui;
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
    selected_task_index: Option<usize>,
}

#[derive(PartialEq)]
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
            selected_task_index: None,
        }
    }

    fn add_task(&mut self) {
        if !self.new_task.is_empty() {
            let task = Task::new(self.new_task.clone());
            self.tasks.push_back(task);
            self.new_task.clear();
        }
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
            .map(|(i, task)| {
                format!(
                    "{}: {} [{}]",
                    i,
                    task.description,
                    if task.completed == TaskStatus::Completed { "x" } else { " " }
                )
            })
            .collect()
    }
}

struct MyApp {
    task_manager: TaskManager,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is where you would load saved tasks from storage if you had persistence
        Self {
            task_manager: TaskManager::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.horizontal(|ui| {
                ui.heading("Task Manager");

                // Filter buttons
                ui.selectable_value(&mut self.task_manager.filter, TaskFilter::All, "All");
                ui.selectable_value(&mut self.task_manager.filter, TaskFilter::Completed, "Completed");
                ui.selectable_value(&mut self.task_manager.filter, TaskFilter::Incomplete, "Incomplete");

                // Search bar (combined with new task input)
                ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| { 
                    ui.add_space(10.0); // Spacing before input
                    ui.text_edit_singleline(&mut self.task_manager.new_task)
                }); 
            });

            // Task Input Area
            ui.horizontal(|ui| {
                if ui.button("Add Task").clicked() {
                    self.task_manager.add_task();
                }
            });

            ui.separator();

            // Task List
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, task) in self.task_manager.display_tasks().iter().enumerate() {
                    let selected = self.task_manager.selected_task_index == Some(i);
                    if ui.selectable_label(selected, task).clicked() {
                        self.task_manager.selected_task_index = Some(i);
                    }
                }
            });

            // Task Details Panel
            if let Some(index) = self.task_manager.selected_task_index {
                ui.separator();
                if let Some(task) = self.task_manager.tasks.get_mut(index) {
                    ui.heading(&task.description);

                    // Allow editing of task description
                    ui.horizontal(|ui| {
                        ui.label("Description:");
                        ui.text_edit_singleline(&mut task.description);
                    });

                    // Button to toggle completion status
                    if ui.button("Toggle Complete").clicked() {
                        if let Err(err) = self.task_manager.toggle_task_complete(index) {
                            eprintln!("{}", err);
                        }
                    }

                    // Remove button
                    if ui.button("Remove").clicked() {
                        if let Err(err) = self.task_manager.remove_task(index) {
                            eprintln!("{}", err);
                        }
                        self.task_manager.selected_task_index = None; 
                    }
                }
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Task Manager",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}
