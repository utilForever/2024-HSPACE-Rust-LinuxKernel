# Exercise: Nested Arrays

Arrays can contain other arrays:

```rust
let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
```

What is the type of this variable?

Use an array such as the above to write a function `transpose` which will
transpose a matrix (turn rows into columns):

```rust
let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
let ret = transpose(array);
// ret == [[1, 4, 7], [2, 5, 8], [3, 6, 9]]
```

Hard-code both functions to operate on 3 × 3 matrices.
