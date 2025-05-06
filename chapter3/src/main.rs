use std::io;

fn main() {
    'main: loop {
        

        loop {
        let mut task = String::new();
        println!("Choose task 1,2 or 3. Choose 0 to exit.");
        
            io::stdin()
                .read_line(&mut task)
                .expect("Failed to read line");

            let task: u8 = match task.trim().parse() {
                Ok(num) => num,
                Err(_) => break println!("Forbidden input try again")
            };

            if task == 0 {
                break 'main;
            } else if task == 1 {
                cel_to_far();
            } else if task == 2 {
                fibonacci();
            } else if task ==3 {
                song();
            } else {
                println!("Forbidden input try again");
            };

        }
    };
}

fn cel_to_far() {
    println!("Please input Farenheit temperature:");

    loop {
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error");
                continue
            }
        };
        
        println!("It's {} Celcius",{(temp-32.0)*5.0/9.0});
        break;
    }
}

fn fibonacci() {
    println!("Please input Fibonacci number id:");

    loop {
        let mut id = String::new();

        io::stdin()
            .read_line(&mut id)
            .expect("Failed to read line");

        let id: usize = match id.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error");
                continue
            }
        };
        let mut n1:usize = 0;
        let mut n2:usize = 1;
        let mut n3:usize = 1;
        if id == 1usize {
            println!("This number is 0")
        } else if id == 2usize {
            println!("This number is 1")
        } else {
            let id = id - 2;

            for _ in (0..id) {
                n3 = n1 + n2;
                n1 = n2;
                n2 = n3;
            };
            println!("This number is {}", n3);
        };
        break;
    }
}

fn song() {
    let things = ["Twelve drummers drumming", "eleven pipers piping",
    "Ten lords a-leaping", "nine ladies dancing", "eight maids a-milking",
    "Seven swans a-swimming", "six geese a-laying", "five gold rings, ba-dum-bum-bum",
    "Four calling birds", "three French hens",
    "Two turtle doves, and", "A partridge in a pear tree"];

    for day in (0..12) {
        println!("On the {}th day of Christmas my true love gave to me",day+1);

        for d in (11-day..12) {
            println!("{}", things[d]);
        };
        println!();
    };
}
