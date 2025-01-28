use core::num;
use std::io::{self, stdin, Write};

#[derive(Debug)]
struct Task{
    id: i32,
    name: String,
}
struct TaskTable{
    tasks: Vec<Task>,
    counter: i32,
}

impl TaskTable {

    fn new() -> Self{
        TaskTable{
            tasks: Vec::new(),
            counter: 1,
        }
    }

    fn add_task_to_list(&mut self){
        loop {
            println!("Podaj zadanie które chcesz wpisać a 'exit' żeby wyjść': ");
            let name = take_the_task();
            if name == "exit"{
                break;
            }
            self.tasks.push(Task {id: self.counter, name});
            println!("Zadanie dodano do listy");
            self.counter += 1;
        }
    }

    fn show_list(&self){
        println!("Lista Zadań");
        if self.tasks.is_empty(){
            println!("Twoja lista jest pusta")
        }
        else {
            for task in &self.tasks{
                println!("{}. {}", task.id, task.name)
            }
        }
    }
}

fn take_the_task() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Błąd odczytu");
    input.trim().to_string()
}

fn take_choice() -> Result<i32, String>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| "Błąd odczytu".to_string())?;
    input.trim().parse().map_err(|_| "Musisz Podać liczbę".to_string())
}


fn main(){
    let mut task_table = TaskTable::new();
    loop {
        println!("Podaj co chcesz zrobić 1-Dodaj zadanie, 2-Pokaż listę, 3-wyjdź");
        let choice = match take_choice() {
            Ok(num) =>num,
            Err(_) => {
                println!("Musisz podać liczbę");
                continue;
            }
        };
        match choice {
            1 => task_table.add_task_to_list(),
            2 => task_table.show_list(),
            3 => {
                println!("Zamykanie programu...");
                break;
            }
            _=> println!("Nie poprawna liczba")
        }
    }
}