/// Головна функція
mod task;
use crate::task::TaskManager;
fn main() {
    let mut manager = TaskManager::load_from_file("tasks.json");

    loop {
        println!("\nМеню:");
        println!("1. Додати завдання");
        println!("2. Видалити завдання");
        println!("3. Оновити завдання");
        println!("4. Позначити завдання як виконане");
        println!("5. Показати всі завдання");
        println!("6. Зберегти та вийти");

        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Помилка вводу");
        let choice = choice.trim();

        match choice {
            "1" => {
                let mut description = String::new();
                println!("Введіть опис завдання:");
                std::io::stdin()
                    .read_line(&mut description)
                    .expect("Помилка вводу");
                let description = description.trim().to_string();
                manager.add_task(description);
            }
            "2" => {
                let mut id = String::new();
                println!("Введіть ID завдання для видалення:");
                std::io::stdin()
                    .read_line(&mut id)
                    .expect("Помилка вводу");
                if let Ok(id) = id.trim().parse::<u32>() {
                    manager.delete_task(id);
                } else {
                    println!("Некоректний ID.");
                }
            }
            "3" => {
                let mut id = String::new();
                let mut new_description = String::new();
                println!("Введіть ID завдання для оновлення:");
                std::io::stdin()
                    .read_line(&mut id)
                    .expect("Помилка вводу");
                if let Ok(id) = id.trim().parse::<u32>() {
                    println!("Введіть новий опис:");
                    std::io::stdin()
                        .read_line(&mut new_description)
                        .expect("Помилка вводу");
                    manager.update_task(id, new_description.trim().to_string());
                } else {
                    println!("Некоректний ID.");
                }
            }
            "4" => {
                let mut id = String::new();
                println!("Введіть ID завдання для позначення як виконаного:");
                std::io::stdin()
                    .read_line(&mut id)
                    .expect("Помилка вводу");
                if let Ok(id) = id.trim().parse::<u32>() {
                    manager.mark_completed(id);
                } else {
                    println!("Некоректний ID.");
                }
            }
            "5" => {
                manager.list_tasks();
            }
            "6" => {
                manager.save_to_file("tasks.json");
                println!("Завдання збережено. Вихід.");
                break;
            }
            _ => {
                println!("Некоректний вибір. Спробуйте ще раз.");
            }
        }
    }
}