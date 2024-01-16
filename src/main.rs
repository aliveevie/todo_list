use rand::Rng;

#[derive(Debug, Clone)]
struct Task {
    id: i64,
    description: String,
    completed: bool,
}

#[derive(Debug)]
struct Tasks {
    todo_list: Vec<Task>,
}

fn add_task(description: &str, tasks: &mut Tasks) -> Task {
    let mut rng = rand::thread_rng();
    let new_id: i64 = rng.gen_range(1..=1000);

    let new_task = Task {
        id: new_id,
        description: String::from(description),
        completed: false,
    };

    // Add the new task to the vector
    tasks.todo_list.push(new_task.clone());

    // Return the created Task instance
    new_task
}

fn complete_task(id: i64, tasks: &mut Tasks) -> Option<&Task> {
    for task in &mut tasks.todo_list {
        if task.id == id {
            task.completed = true;
            return Some(task);
        }
    }
    None
}

fn list_tasks(tasks: &Tasks) {
    for task in &tasks.todo_list {
        println!("Task ID: {}, Description: {}, Completed: {}", task.id, task.description, task.completed);
    }
}

fn main() {
    let mut todo_list = Tasks { todo_list: Vec::new() };

    let new_task = add_task("Do laundry", &mut todo_list);

    println!("Added task: {:?}", new_task);
    println!("Todo list: {:?}", todo_list);
}
