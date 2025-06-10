# Notes

The [Rust Programming Language Book](https://doc.rust-lang.org/stable/book/) is an incredibly comprohensive overview of
everything you could possibly need to know. I find it helpful to keep track on my own, and outline things that my brain
is likely to forget.

## Data Types

### Scalar - Singlular Types

- `Integers`: i8 | u8; i16 | u16; i32 | u32; i64 | u64; i128 | u128; isize | usize;

  - Decimal: 98_222,
  - Hex: 0xff,
  - Octal: 0o77,
  - Binary: 0b1111_0000,
  - Byte (u8 only): b'A'

- `Float`: f32, f64

- `Boolean`: bool

- `Char`: char
  - ASCII

### Compound - Group Types

- `Tuple`: A general grouping of values with a variety of types.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;

let tup: (i32, f64, u8) = (500, 6.4, 1);
let five_hundred = tup.0;
let six_point_four = tup.1;
```

- `Array`: A collection of multiple values of the same type.

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

let b = [3; 5] // [3, 3, 3, 3, 3]

let first = a[0];
let second = b[1];
```
