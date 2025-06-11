use std::io; // standard library, input/output library
use std::io::Write;

// represents individual tasks, "shape" like interfaces in TS
struct Task {
    description: String,
    done: bool,
}

// represents the collection of (Task)s
struct TodoApp {
    tasks: Vec<Task>,
}

impl TodoApp {
    // TodoApp constructor
    fn new() -> TodoApp {
        TodoApp { tasks: Vec::new() }
    }

    fn add_new_task(&mut self, description: &str) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    fn mark_as_done(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.done = true;
        } else {
            println!("No task found at index {}", index + 1);
        }
    }

    fn show_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "[X]" }  else { "[ ]" };
            println!("{} : {} {}", status, index + 1, task.description);
        }
    }

}

impl Task {
    // task constructor
    fn new(description: &str) -> Task { // initializes with the 'new' keyword, taking a description as a parameter
        Task {
            description: description.to_string(),
            done: false, // default status is false, or "not done"
        }
    }
}

fn get_string_input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read the line");
    input.trim().to_string()
}

fn get_numeric_input(prompt: &str) -> Option<u8> {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<u8>().ok()
}

fn main() {
    let mut todo_list_app = TodoApp::new();

    // show menu options
    println!("1. Add new task");
    println!("2. Mark task as done");
    println!("3. Show tasks");
    println!("4. Exit \n ============================= ");

    loop {


        // get user input for menu choice
        let choice = match get_numeric_input("What would you like to do now? \n ============================= ") {
            Some(value) => value,
            None => {
                println!("Invalid input, enter a valid number [1 - 4] \n ============================= ");
                continue;
            }
        };

        match choice {
            1 => {
                let description = get_string_input("Add a task to your list \n =============================  ");
                todo_list_app.add_new_task(&description);
            }
            2 => {
                let index = match get_numeric_input("Enter the task index to mark as done \n =============================  ") {
                    Some(value) => value as usize,
                    None => {
                        println!("Invalid number. only [1 - 4] please \n ============================= ");
                        continue;
                    }
                };
                todo_list_app.mark_as_done(index - 1);
            }
            3 => { 
                todo_list_app.show_tasks(); 
            }
            4 => {
                println!("Goodbye! \n ============================= ");
                break;
            }
            _ => println!("Invalid option, enter number between 1-4 \n ============================= "), // default option, error message
        }
    }
}
