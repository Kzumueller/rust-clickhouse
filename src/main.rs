//extern crate mysql;

use mysql::*;
use mysql::prelude::*;

struct Entry {
    name: String,
    value: f64,
    time: u64,
}


fn main() {
    let pool = Pool::new("mysql://default@localhost:9004/main").unwrap();
    let mut connection = pool.get_conn().unwrap();

    connection.query_drop("CREATE TABLE IF NOT EXISTS `test` (
        `name` String,
        `value` Float64,
        `time` DateTime64(3)
    ) ENGINE=Log").unwrap();

    let entries = vec![
        Entry{name: "AAPL".into(), value: 128.92, time: 1610489275855},
        Entry{name: "AAPL".into(), value: 129.68, time: 1610489275915},
        Entry{name: "AAPL".into(), value: 129.89, time: 1610489275975},
    ];

    connection.exec_batch("INSERT INTO `test`
        (*)
        VALUES (:name, :value, :time)",
        entries.iter().map(|entry| params! {
            ":name" => &entry.name,
            ":value" => entry.value,
            ":time" => entry.time
        })
    ).unwrap();

    let sorted = connection.query_map("SELECT * FROM `test` ORDER BY `value` DESC", |(name, value, time)| {
        Entry {name, value, time}
    }).unwrap();


    println!("highestValue: {} {} {}", sorted[0].value, sorted[0].name, sorted[0].time);
}