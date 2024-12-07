# Advent Of Code 2024

Notes:

## Day1

  A good examples of .filter_map() / .unzip() to parse a stream into two vectors for subsequent independent sorting.

## Day3

  I was stuck until experimented with many_till.

```rust
  fn parse_instr(input: &str) -> IResult<&str, (u32, u32)> {
    let (remain, (_junk, instruction)) = many_till(anychar, parse_mul)(input)?;
    Ok((remain, instruction))
}
```

## day7

  A beautiful example of a handcrafted Generator
  ( Well in RUST I implemented this as a iterator! )
