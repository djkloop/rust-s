// trait bound
trait GetName {
    fn get_name(&self) -> &String;
}

trait GetAge {
    fn get_age(&self) -> u32;
}

struct PeopleMathInfomation<T, U> {
    master: T,
    student: U,
}

impl<T: GetName + GetAge, U: GetName + GetAge> PeopleMathInfomation<T, U> {
    fn print_all_information(&self) {
        println!("master name = {}", self.master.get_name());
        println!("master age = {}", self.master.get_age());
        println!("student name = {}", self.student.get_name());
        println!("student age = {}", self.student.get_age());
    }
}

struct Teacher {
    name: String,
    age: u32,
}

impl GetName for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for Teacher {
    fn get_age(&self) -> u32 {
        self.age
    }
}

struct Student {
    name: String,
    age: u32,
}

impl GetName for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for Student {
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn main() {
    let s = Student {
        name: String::from("student"),
        age: 20,
    };

    let t = Teacher {
        name: String::from("teacher"),
        age: 30,
    };

    let p = PeopleMathInfomation {
        master: t,
        student: s,
    };
    p.print_all_information();

    println!("Hello, world!");
}
