fn main() {

    let person = Person {
        name: String::from("Alex"),
        age: 32,
        sex: true,
        salary: 14342.444,
    };
    println!("{:#?}",person);

    //------------------
    let log = Info (String::from("Success"),1,true);
    println!("{:#?}",log);

}


//simple struct
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    sex: bool,
    salary: f32
}

// cortage struct
#[derive(Debug)]
struct Info (String, u64, bool);