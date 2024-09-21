# Exercise: Mandelbrot Set - Single Thread

This exercise is a simple implementation of the Mandelbrot set using a single thread.

## Context

[The Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set) is the set of complex numbers \(c\) for which the function \(f_{c}(z) = z^{2} + c\) does not diverge when iterated from \(z = 0\), i.e., for which the sequence \(f_{c}(0)\), \(f_{c}(f_{c}(0))\), etc., remains bounded in absolute value. You should implement the function `escape_time()`, `parse_pair()`, `parse_complex()`, `pixel_to_point()`, `render()`, and `write_image()`. You can see each function's description in the `main.rs`.
