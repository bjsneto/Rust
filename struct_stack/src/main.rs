pub struct Stack<T> {
    stack: Vec<T>,
}
  
impl<T> Stack<T> {
  
  #[allow(dead_code)]
  pub fn new() -> Self {
    Stack { 
      stack: Vec::new()
    }
  }

  #[allow(dead_code)]
  pub fn length(&self) -> usize {
    self.stack.len()
  }

  #[allow(dead_code)]
  pub fn pop(&mut self) -> Option<T> {
    self.stack.pop()
  }

  #[allow(dead_code)]
  pub fn push(&mut self, item: T) {
    self.stack.push(item)
  }

  #[allow(dead_code)]
  pub fn is_empty(&self) -> bool {
    self.stack.is_empty()
  }

  #[allow(dead_code)]
  pub  fn peek(&self) -> Option<&T> {
    self.stack.get(self.length() - 1)
  }
}

#[allow(dead_code)]
fn main() {
  
}
