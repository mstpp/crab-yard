fn main() {
    let csv_data = r#"id,name,long
    123,bro,bdata
    4,no,dodo"#;

    let mut rd = csv::ReaderBuilder::new().from_reader(csv_data.as_bytes());

    for line in rd.deserialize::<(u32, String, String)>() {
        println!("{:?}", line);
    }
}
