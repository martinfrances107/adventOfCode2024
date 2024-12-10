# Advent Of Code 2024

Notes:

## Day 1 Zipping numbers

  A good examples of .filter_map() / .unzip() to parse a stream into two vectors for subsequent independent sorting.

## Day 2: Reactor Safety

  Part2: big_gaps_undamped() shows a good used of Itertools::tuple_window()

## Day 3: Scambled computer code

  I was stuck until experimented with many_till.

```rust
  fn parse_instr(input: &str) -> IResult<&str, (u32, u32)> {
    let (remain, (_junk, instruction)) = many_till(anychar, parse_mul)(input)?;
    Ok((remain, instruction))
}
```

## Day 4: X-Wing search

## Day 5: Print Queue

## Day 6: Move the furniture

## Day7: Operators Add, Multiply and Merge

  Part 1:
  A beautiful example of a handcrafted Generator
  ( Well in RUST I implemented this as a iterator! )

  Part 2:
  Operators Add, Mul, Merge

  A good use of Itertools::multi_cartesian_product()
  Future Refactor:
    A run time of 2sec implies that I need to investigate performance.
    All the main loop look like they could be simplified by
    using fold and in particular reduce().
    Also my nom parser could be rewritten.

## Day 8: Frequency planning

  Makes use of Iterrools::cartesian_product.
  The correct data structure is :-

  ```rust
  let map: HashMap<char, HashSet<(usize, usize)>>
  ```

## Day 10: Hiking Routes

  Want to return to this problem and visualize the walk.
