
mod task;

fn check_task(task: &mut crate::task::Task) {
    task.completed = true;
}

fn add_task(task: crate::task::Task, list: &mut crate::task::TodoList) {
   list.tasks.push(task); 
}
