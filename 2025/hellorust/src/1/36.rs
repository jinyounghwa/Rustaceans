use std::fs;
use std::path::Path;
const DATA_FILE: &str = "todos.json";

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
  id: usize,
  title: String,
  completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoList{
  todos: Vec<Todo>,
  next_id: usize,
}

impl TodoList {
  fn new() -> TodoList {
    TodoList {
      todos: Vec::new(),
      next_id: 1,
    }
  }
  fn add(&mut self, title:String){
    let todo = Todo {
      id: self.next_id,
      title,
      completed: false,
    };
    self.todos.push(todo);
    self.next_id += 1;
    println!("Todo 가 추가되었습니다.: {:?}", todo);
  }
  fn list(&self){
    if self.todos.is_empty(){
      println!("Todo 리스트가 비어있습니다.");
      return;
    }
    println!("\n=== TODO 목록 ===");
    for todo in &self.todos {
      let status = if todo.completed { "[x]" } else { "[ ]" };
      println!("{} {} - {}", status, todo.id, todo.title);
    }
    println!();
  }
  fn completed(&mut self, id:usize){
    if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id == id){
      todo.completed = true;
      println!("Todo가 완료되었습니다.");
    }else{
      println!("해당 ID의 Todo를 찾을 수 없습니다.");
    }
  }
  fn remove(&mut self, id:usize){
    if let Some(index) = self.todos.iter().position(|todo| todo.id == id){
      self.todos.remove(index);
      println!("Todo가 삭제되었습니다.");
    }else{
      println!("해당 ID의 Todo를 찾을 수 없습니다.");
    }
  }
  fn save($self) -> Result<(), String>{
    let json = serde_json::to_string(&self)
    .map_err(|e| format!("JSON 직렬화 실패: {}", e))?;
    fs::write(DATA_FILE, json)
    .map_err(|e| format!("파일 저장 실패: {}", e))?;
    println!("Todo 리스트가 저장되었습니다.");
    Ok(())
  }
  fn load() -> Result<TodoList, String>{
   if !Path::new(DATA_FILE).exists(){
    return Ok(TodoList::new());
   }
  let content = fs::read_to_string(DATA_FILE)
  .map_err(|e| format!("파일 읽기 실패: {}", e))?;
  
  serde_json::from_str(&content)
  .map_err(|e| format!("JSON 역직렬화 실패: {}", e))?;
  }
}