use nom::IResult;

/// Represents the elements which can appear in an engine schematic.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SchematicItem {
    Dot,
    Number({value: usize, length: u8}),
    Symbol,
}

fn parse_schematic_line(source: &str) -> IResult<&str, Vec<SchematicItem>> {
    todo!()
}

fn main() {
    println!("Hello, world!");
}
