use core::task;
use std::io::{self, stdin, Write};

#[derive(Debug)]
struct Task{
    id: i32,
    name: String,
    done: bool,
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
            let done = false;
            if name == "exit"{
                break;
            }
            self.tasks.push(Task {id: self.counter, name, done});
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
                if task.done == false{
                    let false_v = "Nie zrobione";
                    println!("{}. {}: {}", task.id, task.name, false_v)
                }
                if task.done == true{
                    let false_v = "Zrobione";
                    println!("{}. {}: {}", task.id, task.name, false_v)
                }
            }
        }
    }
    fn find_task(&mut self, id: i32) -> Option<&mut Task>{
        self.tasks.iter_mut().find(|task| task.id == id)
    }
    
    fn remove_task(&mut self){
        self.show_list();
        println!("WYbierz zadanie które chcesz usunąć");
        let id = take_choice();
        match id {
            Ok(id) => {
                if let Some(index) = self.tasks.iter().position(|task| task.id == id){
                    self.tasks.remove(index);
                    println!("Usunięto zadanie")
                }
            }
            Err(err) => {
                println!("Błąd: {}", err);
            }
        }
    }

    fn change_task_to_done(&mut self){
        self.show_list();
        println!("Wybierz zadanie które ukończyłeś! ");
        let id = take_choice();
        match id{
            Ok(id) => {
                if let Some(task) = self.find_task(id){
                    task.done = true;
                    println!("Zmieniono status zadania! ");
                }
                else {
                    println!("Nie znaleziono zadania")
                }
            }
            Err(err) => {
                println!("Błąd {}", err)
            }
        }    
    }
    fn edit_task(&mut self){
        self.show_list();
        println!("Wybierz zadanie którego nazwę chcesz zmodyfikować");
        let id = take_choice();
        match id {
            Ok(id) => {
                if let Some(task) = self.find_task(id){
                    let new_name = take_the_task();
                    task.name = new_name;
                    println!("Zadanie {} zaktualizowane", id)
                }
                else {
                    println!("Nie znaleziono zadania")
                }
            }
            Err(err) => {
                println!("Błąd {}", err)
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
        println!("Podaj co chcesz zrobić 1-Dodaj zadanie, 2-Pokaż listę, 3-wyjdź, 4-zmień stan zadania na zrobione, 5-usuń zadanie, 6-edytuj zadanie");
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
            4 => task_table.change_task_to_done(),
            5 => task_table.remove_task(),
            6 => task_table.edit_task(),
            _=> println!("Nie poprawna liczba")
        }
    }
}

