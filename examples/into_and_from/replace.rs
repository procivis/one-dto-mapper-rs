use dto_mapper::From;

struct PersonDto {
    name: String,
}

#[derive(From)]
#[convert(from = PersonDto)]
struct FromPerson {
    name: String,

    #[convert(replace = "0u16")]
    age: u16,
}

#[derive(From)]
#[convert(from = PersonDto, into = PersonDto)]
struct IntoFromPerson {
    name: String,

    #[convert(skip)]
    #[convert(replace = "0u16")]
    age: u16,
}

fn main() {
    let p = PersonDto {
        name: "Joe".to_string(),
    };

    let p2 = IntoFromPerson::from(p);
}
