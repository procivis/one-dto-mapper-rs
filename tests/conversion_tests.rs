use dto_mapper::From;

#[test]
fn test_struct() {
    struct SimpleY {
        a: String,
        b: u32,
    }

    #[derive(From)]
    #[convert(from = "SimpleY", into = "SimpleY")]
    struct SimpleX {
        a: String,
        b: u32,
    }

    let x: SimpleX = SimpleY {
        a: "xyz".to_string(),
        b: 10,
    }
    .into();

    let _: SimpleY = x.into();
}

#[test]
fn test_nested_struct() {
    struct Name {
        first: String,
    }

    #[derive(From)]
    #[convert(from = "Name", into = "Name")]
    struct Name2 {
        first: String,
    }

    struct SimpleY {
        a: String,
        n: Name,
    }

    #[derive(From)]
    #[convert(from = "SimpleY", into = "SimpleY")]
    struct SimpleX {
        a: String,
        n: Name2,
    }

    let x: SimpleX = SimpleY {
        a: "xyz".to_string(),
        n: Name {
            first: "Joe".to_string(),
        },
    }
    .into();

    let _: SimpleY = x.into();
}

#[test]
fn test_with_fn_struct() {
    #[derive(Clone)]
    struct Name {
        first: String,
    }

    #[derive(From)]
    #[convert(from = "Name", into = "Name")]
    struct Name2 {
        #[convert(with_fn = "convert_name")]
        first: String,
    }

    fn convert_name(value: String) -> String {
        value
    }

    #[derive(From)]
    #[convert(from = "Name", into = "Name")]
    struct Name3 {
        #[convert(with_fn_ref = "convert_name_ref")]
        first: String,
    }

    fn convert_name_ref(value: &String) -> String {
        value.to_owned()
    }

    let x: Name = Name2 {
        first: "xyz".to_string(),
    }
    .into();

    let _: Name2 = x.to_owned().into();

    let _: Name3 = x.into();
}

#[test]
fn test_unit_enum() {
    enum SimpleY {
        A,
        B,
        C,
    }

    #[derive(From)]
    #[convert(from = "SimpleY", into = "SimpleY")]
    enum SimpleX {
        A,
        B,
        C,
    }

    let x: SimpleX = SimpleY::A.into();

    let _: SimpleY = x.into();
}

#[test]
fn test_newtype_enum() {
    enum SimpleY {
        Id(String),
    }

    #[derive(From)]
    #[convert(from = "SimpleY", into = "SimpleY")]
    enum SimpleX {
        Id(String),
    }

    let x: SimpleX = SimpleY::Id("42".to_string()).into();

    let _: SimpleY = x.into();
}

#[test]
fn test_named_enum() {
    enum SimpleY {
        Name { first: String, last: String },
    }

    #[derive(From)]
    #[convert(from = "SimpleY", into = "SimpleY")]
    enum SimpleX {
        Name { first: String, last: String },
    }

    let x: SimpleX = SimpleY::Name {
        first: "John".to_string(),
        last: "Doe".to_string(),
    }
    .into();

    let _: SimpleY = x.into();
}

#[test]
fn test_mixed_enum() {
    enum SimpleY {
        A,
        Id(String),
        Named { x: String, y: i64 },
    }

    #[derive(From)]
    #[convert(from = "SimpleY", into = "SimpleY")]
    enum SimpleX {
        A,
        Id(String),
        Named { x: String, y: i64 },
    }

    let x: SimpleX = SimpleY::Id("42".to_string()).into();

    let _: SimpleY = x.into();
}
