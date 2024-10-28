#[derive(ximple::ToXml)]
struct Foo {
    #[ximple = "foo"]
    foo: usize,
}

fn main() {}
