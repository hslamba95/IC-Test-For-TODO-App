
// Define a struct for a TODO item
#[derive(Clone)]
struct Todo {
    id: String,
    text: String,
}

impl Todo {
    fn new(text: String) -> Self {
        // Generate a random ID for the new TODO item
        let id = generate_random_id();
        Todo { id, text }
    }
}

// Global variable to store TODO items
static mut TODOS: Vec<Todo> = Vec::new();

// Function to generate a random ID for a TODO item
fn generate_random_id() -> String {
    // Placeholder implementation to generate a random ID
    "hslamba".to_string()
}

// Function to add a new TODO item
fn add_todo(text: String) -> String {
    let new_todo = Todo::new(text);
    unsafe {
        TODOS.push(new_todo.clone());
    }
    format!("Todo Struct Saved: ID: {}, Text: {}", new_todo.id, new_todo.text)
}

// Function to read a TODO item by ID
fn read_todo_by_id(id: &str) -> Option<&Todo> {
    unsafe {
        TODOS.iter().find(|&todo| todo.id == id)
    }
}

// Function to update a TODO item by ID
fn update_todo(id: &str, new_text: String) -> bool {
    if let Some(todo) = unsafe { TODOS.iter_mut().find(|todo| todo.id == id) } {
        todo.text = new_text;
        true
    } else {
        false
    }
}

// Function to delete a TODO item by ID
fn delete_todo(id: &str) -> bool {
    if let Some(index) = unsafe { TODOS.iter().position(|todo| todo.id == id) } {
        unsafe {
            TODOS.remove(index);
        }
        true
    } else {
        false
    }
}

fn check_todos() -> bool {
    let todos_len = unsafe { TODOS.len() };
    todos_len > 0
}

// Define endpoint for adding a new TODO item
#[ic_cdk::query]
fn add_todo_endpoint(text: String) -> String {
    add_todo(text)
}

// Define endpoint for reading a TODO item by ID
#[ic_cdk::query]
fn read_todo_by_id_endpoint(id: String) -> String {
    match read_todo_by_id(&id) {
            Some(todo) => format!("Todo found: ID: {}, Text: {}", todo.id, todo.text),
            None => format!("Todo with ID {} not found", id),
        }
}

// Define endpoint for updating a TODO item by ID
#[ic_cdk::query]
fn update_todo_endpoint(id: String, new_text: String) -> bool {
    update_todo(&id, new_text)
}

#[ic_cdk::query]
fn check_len_endpoint() -> bool {
    check_todos()
}

// Define endpoint for deleting a TODO item by ID
#[ic_cdk::query]
fn delete_todo_endpoint(id: String) -> bool {
    delete_todo(&id)
}