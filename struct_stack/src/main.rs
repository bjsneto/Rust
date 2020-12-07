struct Stack<T> {
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

fn main() {
 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_empty() {
      let pilha: Stack<isize> = Stack::new();
      assert_eq!(pilha.is_empty(), true);
    }

    #[test]
    fn length() {
      let pilha: Stack<isize> = Stack::new();
      assert_eq!(pilha.length(), 0);
    }

    #[test]
    fn push() {
      let mut pilha: Stack<isize> = Stack::new();
      assert_eq!(pilha.push(1), ());
    }

    #[test]
    fn pop() {
      let mut pilha: Stack<isize> = Stack::new();
      assert_eq!(pilha.push(1), ());
      assert_eq!(pilha.push(2), ());
      assert_eq!(pilha.push(3), ());
      assert_eq!(pilha.pop(), Some(3));
    }
    
    #[test]
    fn peek() {
      let mut pilha: Stack<isize> = Stack::new();
      assert_eq!(pilha.push(1), ());
      assert_eq!(pilha.push(2), ());
      assert_eq!(pilha.push(3), ());
      assert_eq!(pilha.peek(), Some(&3));
    }
}

