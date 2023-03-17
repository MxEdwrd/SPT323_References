//References - Task Management App
//Max Edward | 3/16/23

use std::io; //Standard library module for in/out operations

struct Task {
    //Constructor for Task
    title: String,       //title string for name of task
    description: String, //description string for description of task
    completed: bool,     //completion boolean for assigning completion of task
}

impl Task {
    //Implementation of Task
    //Method for creating a task with title, description, and gives it default completion of false
    fn new_task(title: String, description: String) -> Task {
        Task {
            title,
            description,
            completed: false,
        }
    }

    //Method for assigning completion of task
    //Create reference to current object and modify completion status
    fn complete_task(&mut self) {
        self.completed = true;
    }

    //Method for displaying task, description, and completion status
    //Reference current object, non-mutable
    fn display_task(&self) {
        println!("Title: {}", self.title);
        println!("Description: {}", self.description);
        println!("Completed: {}", self.completed);
    }
}

fn main() {
    //Initialize vector to begin storing tasks
    let mut tasks: Vec<Task> = Vec::new();

    //Greet user
    println!("\n\n\nWelcome to MaxTask - The Task Management Application");

    //If there are no tasks in vector, print out accompanying lines.
    if tasks.is_empty() {
        println!("\n------------------------------------------\n");
        println!("Current Tasks:\n\nNo Tasks\n");

    //If there are tasks, display task details
    //Shouldn't run normally as there no tasks when program starts
    } else {
        println!("\n------------------------------------------\n");
        println!("Current Tasks:\n");
        for task in &tasks {
            task.display_task();
        }
        println!();
    }

    //Beginning of loop
    loop {
        //Read out user options
        println!("------------------------------------------\n");
        println!("Enter a command: add, complete, remove, or quit\n");

        //Take in user input, store in reference of input variable
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        //Match case statement for user input
        match input.trim() {
            //If "add", take user input for reference of title, description, add it to task vector
            "add" => {
                println!("\nEnter a title:\n");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("\nFailed to read input");

                println!("\nEnter a description:\n");
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("\nFailed to read input");

                let task = Task::new_task(title.trim().to_string(), 
                description.trim().to_string());
                tasks.push(task);
            }
            //If "complete", take user input for title of task, modify task completion status,
            //return new completion status
            "complete" => {
                println!("\nEnter the title of the task to complete:\n");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("\nFailed to read input");

                //Reference tasks to compare titles for completion assignment
                if let Some(task) = tasks.iter_mut().find(|t| 
                    t.title.trim() == input.trim()) {
                    task.complete_task();
                    println!("\nTask '{}' completed", task.title);
                } else {
                    println!("\nNo task with title '{}' found", input.trim());
                }
            }
            //If "remove", take user input for title of task, index the task, and remove it 
            //from list
            "remove" => {
                println!("\nEnter the title of the task to remove:\n");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("\nFailed to read input");

                //Reference tasks to compare titles for task removal
                if let Some(index) = tasks.iter().position(|t| 
                    t.title.trim() == input.trim()) {
                    tasks.remove(index);
                    println!("\nTask '{}' removed", input.trim());
                } else {
                    println!("\nNo task with title '{}' found", input.trim());
                }
            }
            //If "quit", break out of loop, end program
            "quit" => break,
            _ => println!("Invalid command"),
        }

        // Display all tasks after user input
        //If there are no more tasks, print out accompanying lines
        if tasks.is_empty() {
            println!("\n------------------------------------------\n");
            println!("Current Tasks:\n\nNo Tasks\n");
        //If there are still tasks, display task details
        } else {
            println!("\n------------------------------------------\n");
            println!("Current Tasks:\n");
            for task in &tasks {
                task.display_task();
            }
            println!();
        }
    }
}
