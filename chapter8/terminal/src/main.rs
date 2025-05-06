use std::io;
use std::collections::HashMap;

fn main() {
    let mut database: HashMap<String,Vec<String>> = HashMap::new();

    loop {
        println!("\n    Welcome to the terminal!\n
    You can use one of these commands:\n
    add, check, exit\n
    After you choose action you'll be given more options connected to action\n"
        );

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        command = command.trim().to_lowercase().to_string();
        
        match command.as_str() {
            //"edit" => edit_person(&mut database),
            //"remove" => remove_person(&mut database),
            "check" => check_database(&mut database),
            "add" => add_person(&mut database),
            "exit" => break,
            _ => println!("Unknown command")
        };
    };


}

fn add_person(database:&mut HashMap<String,Vec<String>>) {
    println!("\n    You can add a person to a department following the example below: \n
    Department_Name;Person_Name\n"
    );

    let mut data = String::new();
    io::stdin()
        .read_line(&mut data)
        .expect("Failed to read line");
        
    let index = match data.find(";") {
        Some(index) => index,
        None => 999999,
    };

    if index<1 || index == 999999{
        println!("\n    Please follow example\n");
    } else {
        let department = &data[0..index].trim();
        let name = &data[index+1..].trim();
        match database.get_mut(*department) {
            Some(department_data) => department_data.push(name.to_string()),
            _ => { database.entry(department.to_string()).or_insert(vec![name.to_string()]); }
        };
        
    };
}

fn check_database(database:&mut HashMap<String,Vec<String>>) {
    println!("\n    Your database is looking like this:\n");

    let mut vec:Vec<String> = database.clone().into_keys().collect();
    vec.sort();

    for department in vec {
        println!("\nDepartment: {}\n\n",department);

        let mut names:Vec<String> = match database.get(&department) {
            Some(value) => value.clone(),
            _ =>vec![String::from("There's no way")]
        };
        
        names.sort();
        for name in names {
            println!("Worker: {name}\n");
        };
    };

}

//fn remove_person(database:&mut HashMap<String,Vec<String>>) {
//    println!("removing");
//}

//fn edit_person(database:&mut HashMap<String,Vec<String>>) {
//    println!("editing");
//}

