# Exercise: Mandelbrot Set - Rayon

This exercise is a simple implementation of the Mandelbrot set using `rayon` library.

## Context

The standard library's spawn function is an important primitive, but it's not designed specifically for fork-join parallelism. Better fork-join APIs have been built on top of it. One of the most popular is the `rayon` library, which provides a data parallelism API. It allows you to parallelize operations on collections, such as arrays and vectors, with minimal effort. You should change the code to use `rayon` library to calculate the Mandelbrot set faster.
