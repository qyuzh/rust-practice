trait UsernameWidget {
    // Get the selected username out of this widget
    fn get(&self) -> String;
}

trait AgeWidget {
    // Get the selected age out of this widget
    fn get(&self) -> u8;
}

// A form with both a UsernameWidget and an AgeWidget
struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "qyuzh".to_owned(),
        age: 26,
    };

    // println!("{}", form.get()); // try to uncomment
    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("qyuzh".to_owned(), username);

    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(26, age);
}
