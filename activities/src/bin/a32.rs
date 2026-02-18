// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

#[derive(Debug)]
#[allow(dead_code)]
struct Person<'a> {
    name: &'a str,
    title: &'a str,
}

fn parse_persons(data: &str) -> Vec<Person<'_>> {
    data.lines()
        .skip(1) // skip header row
        .filter_map(|line| {
            let mut fields = line.split(',');
            let _id = fields.next()?;
            let name = fields.next()?;
            let _email = fields.next()?;
            let _dept = fields.next()?;
            let title = fields.next()?;
            Some(Person { name, title })
        })
        .collect()
}

fn main() {
    let persons = parse_persons(MOCK_DATA);
    for person in &persons {
        println!("{}: {}", person.name, person.title);
    }
}
