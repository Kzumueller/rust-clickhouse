extern crate mysql;

use mysql::*;
use mysql::prelude::*;

struct Entry {
    name: String,
    value: f64,
    time: i64,
}


fn main() {
    let pool = Pool::new("mysql://default@localhost:9004/main").unwrap();
    let mut connection = pool.get_conn().unwrap();

    connection.query_drop("CREATE TABLE IF NOT EXISTS `test` (
        `name` String,
        `value` Float64,
        `time` Int64
    ) ENGINE=Log").unwrap();

    println!("Table created");

    let entries = vec![
        Entry{name: "AAPL".into(), value: 128.92, time: 1610489275855},
        Entry{name: "AAPL".into(), value: 129.68, time: 1610489275915},
        Entry{name: "AAPL".into(), value: 129.89, time: 1610489275975},
    ];
    
    let value_statements : Vec<String> = entries.iter().map(|entry| {
        format!("('{}', {}, {})", entry.name, entry.value, entry.time)
    }).collect();
    
    let test = format!("INSERT INTO `test` (name, value, time) VALUES {}", value_statements.join(","));

    println!("{}", test);

    connection.query_drop(
        format!("INSERT INTO `test` (name, value, time) VALUES {}", value_statements.join(","))
    );

    println!("data inserted");

    let sorted : std::vec::Vec<Entry> = connection.query_map("SELECT * FROM `test` ORDER BY `value` DESC LIMIT 100", |(name, value, time)| {
        Entry { name, value, time }
    }).unwrap();


    for entry in sorted {
        println!("{} {} {}", entry.name, entry.value, entry.time);
    }
}