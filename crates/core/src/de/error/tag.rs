#[derive(Debug)]
pub enum Tag {
    Start,
    End,
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Tag::Start => "start",
            Tag::End => "end",
        })
    }
}
