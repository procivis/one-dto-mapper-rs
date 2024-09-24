use one_dto_mapper::From;

struct NumericDto {
    age: u16,
    height: u32,
}

#[derive(From)]
#[from(NumericDto)]
struct StringDto {
    #[from(with_fn = "custom_to_string")]
    age: String,

    #[from(with_fn_ref = "custom_to_string_from_ref")]
    height: String,
}

fn custom_to_string(value: u16) -> String {
    value.to_string()
}

fn custom_to_string_from_ref(value: &u32) -> String {
    value.to_string()
}

fn main() {
    let p = NumericDto {
        age: 42,
        height: 200,
    };

    let _p2 = StringDto::from(p);
}
