# Exercise: for_2d!

In this task, we'll be implementing code to make the following syntax possible:

```rust
fn main() {
    for_2d!(row <i32> in 1..5, col <i32> in 2..7, {
        // code
    });
}
```

Ignoring extra curly braces, this code should translate to

```rust
fn main() {
    for row in 1..5 {
        let row: i32 = row;
      
        for col in 2..7 {
            let col: i32 = col;
         
            // Code
        }
    }
}
```

Note that the names of the variables may change (i.e. they could be `row` and `col`, or `x` and `y`, or something else).

To complete this task, there more fragment specifiers you will need to know about:

- `ident`: an "identifier", like a variable name. `ident` meta-variables. Can be followed by anything.
- `block`: a "block expression" (curly braces, and their contents). Can be followed by anything.
- `ty`: a type. Can only be followed by `=>`, `,`, `=`, `|`, `;`, `:`, `>`, `>>`, `[`, `{`, `as`, `where`, or a `block` meta-variable.
