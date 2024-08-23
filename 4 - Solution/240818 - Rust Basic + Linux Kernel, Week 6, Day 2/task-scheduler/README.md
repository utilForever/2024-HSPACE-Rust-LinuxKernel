# Exercise: State-Tracking Task Scheduler with Dynamic Closures

Design a Task Scheduler with closures that manage stateful operations. Each task should be represented as a closure that can maintain internal state between invocations. Some tasks may accumulate values, keep track of the number of invocations, or modify shared data.

Your task is to implement a Task Scheduler that supports:

1. Stateful tasks that can track their own state.
2. Shared mutable state across tasks using `Rc<RefCell<T>>`.
3. Conditional execution based on task results: If a task returns "fail", stop execution.
4. A method to restart the scheduler and re-run all tasks from the beginning, preserving state where appropriate.
