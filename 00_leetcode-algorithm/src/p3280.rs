pub fn convert_date_to_binary(date: String) -> String {
    let splits = date.split('-').collect::<Vec<&str>>();
    let mut output = vec![];
    for t in splits.iter() {
        let mut num = t.parse::<i32>().unwrap();
        let binary = format!("{:b}", num);
        output.push(binary);
    }
    output.join("-")
}
