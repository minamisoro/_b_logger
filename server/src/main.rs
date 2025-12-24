pub mod db;
pub mod schema;

fn main() {
    println!("Hello, world!");

    // Example: Establish database connection
    let _connection = &mut db::establish_connection();
    println!("Successfully connected to database!");
}
