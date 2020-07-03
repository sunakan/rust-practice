use std::thread;
use std::time::Duration;

fn main() {
  //test01();
  //test02();
  test03();
}

fn test01() {
    thread::spawn(|| {
        for i in 1..10 {
            thread::sleep(Duration::from_millis(1));
            // やあ！立ち上げたスレッドから数字{}だよ！
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..5 {
        // メインスレッドから数字{}だよ！
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(3));
    }

}

fn test02() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            thread::sleep(Duration::from_millis(1));
            // やあ！立ち上げたスレッドから数字{}だよ！
            println!("hi number {} from spawnすれっど", i);
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        // メインスレッドから数字{}だよ！
        println!("hi number {} from the めいん thread!", i);
        thread::sleep(Duration::from_millis(3));
    }
}

fn test03() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}
