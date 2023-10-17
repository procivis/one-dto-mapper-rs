use dto_mapper::From;

struct Tuple2(u32);

#[derive(From)]
#[convert(from = "Tuple2")]
struct Tuple(u32);

fn main() {}