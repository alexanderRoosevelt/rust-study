fn main() {

    // let person = Person {
    //     name: String::from("Alex"),
    //     age: 32,
    //     sex: true,
    //     salary: 14342.444,
    // };
    // println!("{:#?}",person);
    //
    // //------------------
    // let log = Info (String::from("Success"),1,true);
    // println!("{:#?}",log);

    let triangle = Triangle {
        c1: 3.0,
        c2: 4.0,
    };
    println!("hypotenuza is {}", triangle.find_hyp())
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

struct Triangle {
    c1: f32,
    c2: f32
}

// implement method in struct
impl Triangle {
    fn find_hyp(self) -> f32{
        (self.c1 * self.c1 + self.c2 * self.c2).sqrt()
    }
}

