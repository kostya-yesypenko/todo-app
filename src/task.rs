use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: u32,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: u32, description: String) -> Self {
        Task {
            id,
            description,
            completed: false,
        }
    }
}

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }

    /// Додати нове завдання
    pub fn add_task(&mut self, description: String) {
        let id = (self.tasks.len() as u32) + 1;
        self.tasks.push(Task::new(id, description));
    }

    /// Видалити завдання за ID
    pub fn delete_task(&mut self, id: u32) {
        self.tasks.retain(|task| task.id != id);
    }

    /// Оновити опис завдання
    pub fn update_task(&mut self, id: u32, new_description: String) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.description = new_description;
        }
    }

    /// Позначити завдання як виконане
    pub fn mark_completed(&mut self, id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.completed = true;
        }
    }

    /// Зберегти завдання у файл
    pub fn save_to_file(&self, filename: &str) {
        let serialized = serde_json::to_string(&self.tasks).expect("Помилка серіалізації");
        fs::write(filename, serialized).expect("Помилка запису у файл");
    }

    /// Завантажити завдання з файлу
    pub fn load_from_file(filename: &str) -> Self {
        let data = fs::read_to_string(filename).unwrap_or_else(|_| "[]".to_string());
        let tasks: Vec<Task> = serde_json::from_str(&data).expect("Помилка десеріалізації");
        TaskManager { tasks }
    }

    /// Вивести список завдань
    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("Список завдань порожній.");
        } else {
            for task in &self.tasks {
                println!(
                    "[{}] {} - {}",
                    task.id,
                    if task.completed { "✔" } else { " " },
                    task.description
                );
            }
        }
    }
}