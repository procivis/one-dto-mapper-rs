use dto_mapper::From;

struct NumericDto {
    age: u16,
}

#[derive(From)]
#[convert(from = NumericDto)]
struct StringDto {
    #[convert(with_fn = "custom_to_string", with_fn_ref = "custom_to_string_from_ref")]
    age: String,
}

fn custom_to_string(value: u16) -> String {
    value.to_string()
}

fn custom_to_string_from_ref(value: &u16) -> String {
    value.to_string()
}

fn main() {
}