mod main;
#[cfg(test)]
use main::*;
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

