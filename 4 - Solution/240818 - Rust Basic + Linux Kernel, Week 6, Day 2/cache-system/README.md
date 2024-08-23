# Exercise: The Cache System

Use closures to implement a system that caches the results of a function to save CPU or network resources. The given `cached_function` function should store the result of a calculation in memory, and when the same input comes in, it should return the previous result without recalculating.

In this problem, you will implement the cache using a closure, and since you need to change the internal state, you should use the `FnMut` closure.
