extern crate mysql;

use mysql::*;

fn main() {
    println!("trying to establish connection");

    let pool = Pool::new("mysql://default@localhost:9004/main");

    println!("Connection established!");
}