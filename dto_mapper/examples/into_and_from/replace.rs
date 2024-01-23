use dto_mapper::{From, Into};

struct PersonDto {
    name: String,
}

#[derive(From)]
#[from(PersonDto)]
struct FromPerson {
    name: String,

    #[from(replace = "0u16")]
    age: u16,
}

#[derive(Into, From)]
#[from(PersonDto)]
#[into(PersonDto)]
struct IntoFromPerson {
    name: String,

    #[into(skip)]
    #[from(replace = "0u16")]
    age: u16,
}

fn main() {
    let p = PersonDto {
        name: "Joe".to_string(),
    };

    let p2 = IntoFromPerson::from(p);
}
