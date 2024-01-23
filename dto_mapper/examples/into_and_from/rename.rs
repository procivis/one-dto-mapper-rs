use dto_mapper::{From, Into};

struct PersonDto {
    name: String,
}

#[derive(Into, From)]
#[into(PersonDto)]
#[from(PersonDto)]
struct Person {
    #[into(rename = "name")]
    #[from(rename = "name")]
    full_name: String,
}

fn main() {
    let p = Person {
        full_name: "Joe".to_string(),
    };

    let p2 = PersonDto::from(p);
    let _p3 = Person::from(p2);
}
