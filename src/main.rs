fn main() {
    println!("Hello, world!");
    foo_bar_printer();
}

fn foo_bar_printer() { 
    for i in 0..100 {
//        print!("{}",i);
        let mut printed: bool = false;
        if i % 3 == 0 {
            print!("foo");
            printed = true;
        }

        if i % 5 == 0 {
            print!("bar");
            printed = true;
        }
        
        if printed {
            println!();
        }
    }
}
