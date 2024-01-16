use rand::Rng;

#[derive(Debug, Clone)]
#[warn(dead_code)]
struct Task{
    id: i64,
    description: String,
    completed: bool
}

#[derive(Debug)]
struct Tasks{
    todo_list: Vec<Task>
}


fn add_task(description: &str, task: &mut Tasks) -> Task{
    let mut rng = rand::thread_rng();
    let new_id: i64 = rng.gen_range(1..1000);

    let new_task: Task = Task {
        id: new_id,
        description: String::from(description),
        completed: false,
    };

    task.todo_list.push(new_task.clone());
    // Return the created Task instance
    return new_task;
}

fn complete_task(id: i64, task: &mut Tasks) -> Option<&Task>{
    for task in &mut task.todo_list{
        if task.id == id {
            task.completed = true;
            return Some(task);
        }
    }

    None
}

fn list_task(){
    let task = Tasks {
        todo_list: Vec::new()
    };
        for task in &task.todo_list{
            println!(
                "Task ID: {}, Description: {}, Completed: {}",
                task.id, task.description, task.completed
            );
        }
}

fn main() {
    let mut current_tasks: Tasks = Tasks {
        todo_list: Vec::new()
    };

    let new_task = add_task("Girl!", &mut current_tasks);

  // println!("The current tasks are {:#?}", current_tasks);
  // println!("The New tasks are {:#?}", new_task);
   list_task();



   
}
