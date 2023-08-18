# better-num

Rust numeric types that are easier to use
> Note: due to space constraints, anything > 16 bits cannot be implemented, as it would take multiple gigabytes of space

> NOTE: `rustc` or `rust-analyzer` may not like the size of some of these enum definitions, you have been warned.


## Example:
```rust
use better_num::english::u8::U8English::{TwoHundredFiftyFour, TwoHundredThirtyFour};

fn main() {
    dbg!(TwoHundredFiftyFour - TwoHundredThirtyFour);

    // Twenty
}

```
Check the [examples](https://github.com/adryzz/better-num/tree/master/examples) folder for more examples.

Due to the large size of the definitions, and to improve compilation speed you have to select exactly the types you want to use as features.

For example, if i want the `U16English` type, i have to select the features `english` and `u16`.

## Language support

- [x] English
- [ ] French
- [ ] German
- [ ] Italian
> more to be added

## License

This code is licensed under the MIT license.