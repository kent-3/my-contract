use serde::Serialize;

#[derive(Serialize)]
pub struct YourStruct {
    pub id: i32,
    pub name: String,
}

fn main() {
    // Create an instance of your struct
    let data = YourStruct {
        // Fill in your struct fields here
        id: 1,
        name: "bob".to_string(),
    };

    // Serialize your data to a compact JSON string
    let j = serde_json_wasm::to_string(&data).unwrap();

    // Print the JSON string
    println!("{}", j);

    // Customize Formatter to use 4 spaces for indents
    let mut formatter = jsonxf::Formatter::pretty_printer();
    formatter.indent = String::from("    ");

    // Transform the JSON string to be pretty
    let p = formatter.format(&j).unwrap();

    // Print the JSON string
    println!("{}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        main();

        let mut formatter = jsonxf::Formatter::pretty_printer();
        formatter.indent = String::from("    ");
        formatter.trailing_output = String::from("\n");

        let ugly_json = "{\"hello\":\"world\"}";
        let pretty_json = formatter.format(ugly_json).unwrap();
        assert_eq!(pretty_json, "{\n    \"hello\": \"world\"\n}\n");
    }
}
