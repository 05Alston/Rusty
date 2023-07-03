fn main() {
    println!("Hello, world!");
    fizz_buzz();
    foo_bar();
}

fn fizz_buzz() {
    let mut iter = 1;
    while iter < 100 {
      let mut msg = String::new(); 
      if iter % 3 == 0 {
          msg = msg.to_owned() + "fizz";
      }
      if iter % 5 == 0 {
          msg = msg.to_owned() + "buzz";
      }
      if msg == "" {
          msg = iter.to_string();
      }
        
      println!("{msg}");
      iter += 1;
    }
}

fn foo_bar() {
    for i in 1..101 {
        match (i % 3, i % 5) {
            (0, 0) => {println!("Foo Bar");},
            (0, _) => {println!("Foo");},
            (_, 0) => {println!("Bar");},
            (_, _) => {println!("{i}");},
        }   
    }
}
