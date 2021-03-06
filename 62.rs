use Tree::{Empty, Node};

#[derive(Clone, PartialEq)]
enum Tree<T> {
  Empty,
  Node(T, Box<Tree<T>>, Box<Tree<T>>)
}

fn internals<T: PartialEq>(t: &Tree<T>) -> usize {
  match *t {
    Empty => 0,
    Node(_, ref l, ref r) =>
      if **l == Empty && **r == Empty {
        0
      } else {
        1 + internals(&**l) + internals(&**r)
      }
  }
}

fn main() {
  let e = Box::new(Empty);
  let a1 = Box::new(Node(1, e.clone(), e.clone()));
  let a2 = Box::new(Node(2, a1.clone(), e.clone()));
  let a3 = Box::new(Node(2, e.clone(), a1.clone()));
  let a = Node(1, a2.clone(), a2.clone());
  println!("{}", internals(&a));
}
