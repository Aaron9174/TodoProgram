
mod task;

pub todo_list: TodoList = []

pub fn add_task(task: Task) {
    todo_list.tasks.push(task);
}
