use std::collections::HashMap;

fn main() {
    
    let array = [1,2,3,4,1,2,1,2,1,1,6,9,99,7,1,1,1,6,6,7,8,4,3];
    let array = [1,2,2,2,2,3,3,3,3,4];
    let mut vec:Vec<isize> = Vec::new();
    let mut counter = 0.0;
    let mut su = 0.0;
    let mut flag = true;
    let mut mean = 0.0;
    let mut mode = HashMap::new();

    for i in array {
        flag = !flag;
        su +=i as f64;
        vec.push(i);
        counter += 1.0;

        let count = mode.entry(i).or_insert(0);
        *count += 1;
    };

    vec.sort();
    if flag {
        mean = (vec[(counter/2.0 - 1.0) as usize] + vec[(counter/2.0) as usize]) as f64/2.0;
    } else {
        mean = vec[(counter/2.0 - 0.5) as usize] as f64;
    };

    let mut maxval = (0,0);
    for (key,val) in mode {
        if maxval.1 < val {
            maxval = (key,val);
        }
    };


    println!("avg = {}, mean = {}, mode = {}", su/counter, mean, maxval.0);
}
