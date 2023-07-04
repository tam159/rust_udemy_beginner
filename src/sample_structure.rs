pub fn example_structure() {
    let emp = Employee {
        name: String::from("Tam"),
        company: String::from("Google"),
        age: 35,
    };

    println!("{:?}", emp);
    println!("{}", emp.name);
    println!("{}", emp.get_details());
    println!("{}", Employee::static_details());
}

#[derive(Debug)]
struct Employee {
    name: String,
    company: String,
    age: u32,
}

impl Employee {
    fn get_details(&self) -> String {
        format!(
            "name: {}, age: {}, company: {}",
            &self.name, &self.age, &self.company
        )
    }

    fn static_details() -> String {
        String::from("Details of a person")
    }
}
