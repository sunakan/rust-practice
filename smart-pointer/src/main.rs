#[derive(Debug)]
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}
enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
  //test_01();
  //test_02();
  //test_03();
  //test_04();
  //test_05();
  //test_06();
  //test_07();
  //test_08();
  test_09();
}

fn test_01() {
    let b = Box::new(5);
    println!("b = {}", b);
}

fn test_02() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}

fn test_03() {
  let x = 5;
  let y = &x;
  println!("--> x={}", x);
  println!("--> &x={:p}", &x);
  println!("--> y={:p}", y);
  println!("--> *y={}", *y);
  println!("--> &y={:p}", &y);
}

fn test_04() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    println!("--> {:?}", y);
    println!("--> {:p}", &y);
    println!("--> {:p}", y.deref());
    println!("--> {}", &y.0);
    println!("--> {}", *y);
    println!("--> {}", *(y.deref()));
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn test_05() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // CustomSmartPointerをデータ`{}`とともにドロップするよ
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn test_06() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}

fn test_07() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    //c.drop();
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

fn test_08() {
    //let a = Cons(5,
    //    Box::new(Cons(10,
    //        Box::new(Nil))));
    //let b = Cons(3, Box::new(a));
    //let c = Cons(4, Box::new(a));
    let a = Rc::new(List2::Cons(5, Rc::new(List2::Cons(10, Rc::new(List2::Nil)))));
    let b = List2::Cons(3, Rc::clone(&a));
    let c = List2::Cons(4, Rc::clone(&a));
}

fn test_09() {
    let a = Rc::new(List2::Cons(5, Rc::new(List2::Cons(10, Rc::new(List2::Nil)))));
    // a生成後のカウント = {}
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = List2::Cons(3, Rc::clone(&a));
    // b生成後のカウント = {}
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = List2::Cons(4, Rc::clone(&a));
        // c生成後のカウント = {}
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    // cがスコープを抜けた後のカウント = {}
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
