extern crate swapi;

fn main() {
    println!("Requesting a person");
    println!("data response is {:?}", swapi::types::people::query_people("10"));
}
