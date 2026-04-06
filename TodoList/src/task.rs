#[derive(Debug)]
struct Task
{
    pub id: u32,
    pub name: String;
    pub description: String,
    pub completed: bool
}

impl Task
{
    pub fn complete_task(&mut self, state: bool) {
        self.completed = state;
    }
}

#[derive(Debug)]
pub struct TodoList
{
    pub tasks: Vec<Task>
}



