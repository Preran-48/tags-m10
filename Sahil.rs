use boilerplate::Boilerplate;

    #[derive(Boilerplate)]
    #[boilerplate(path = "templates/my_template.txt")]
    struct MyData {
        name: String,
        age: u8,
    }

    fn main() {
        let data = MyData {
            name: "Alice".to_string(),
            age: 30,
        };
        println!("{}", data); // Outputs the rendered template
    }
