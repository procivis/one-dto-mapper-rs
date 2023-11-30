use dto_mapper::TryFrom;

struct NumericDto {
    age: u16,
    height: u32,
}

#[derive(TryFrom)]
#[try_from(T = NumericDto, Error = String)]
struct StringDto {
    #[try_from(with_fn = "custom_to_string")]
    age: String,

    #[try_from(with_fn_ref = "custom_to_string_from_ref")]
    height: String,
}

fn custom_to_string(value: u16) -> Result<String, String> {
    Ok(value.to_string())
}

fn custom_to_string_from_ref(value: &u32) -> Result<String, String> {
    Ok(value.to_string())
}

fn main() {
    let p = NumericDto {
        age: 42,
        height: 200,
    };

    let _p2 = StringDto::try_from(p).unwrap();
}
