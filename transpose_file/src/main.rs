use std::fs;
//use std::io;

fn main() {

    let file = fs::read_to_string("file.txt").expect("fail");

    let rows = file.lines();
    let mut colums = 0;
    for r in rows.clone() {
        for _ in r.split_whitespace(){
            colums+=1
        };
        break;
    };
    let mut output = vec![String::new();colums];
    
    for r in rows {
        let mut counter = 0;
        for i in r.split_whitespace() {
            output[counter] = output[counter].clone()+i;
            output[counter].push(' ');
            counter+=1;
        };
    };
    for i in 0..colums {
        println!("{}",output[i].trim());
    };
    
}
