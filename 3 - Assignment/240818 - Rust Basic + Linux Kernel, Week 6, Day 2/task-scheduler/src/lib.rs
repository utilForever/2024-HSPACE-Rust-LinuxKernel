use std::cell::RefCell;
use std::rc::Rc;

type SharedState = Rc<RefCell<i32>>;
type Task = Box<dyn FnMut() -> String>;

#[allow(dead_code)]
pub struct TaskScheduler {
    tasks: Vec<Task>,
    shared_state: SharedState,
}

impl TaskScheduler {
    // Create a new TaskScheduler with shared state initialized to 0.
    pub fn new() -> Self {
        todo!()
    }

    // Register a stateful task.
    pub fn register_task<F>(&mut self, _task: F)
    where
        F: FnMut() -> String + 'static,
    {
        todo!()
    }

    // Execute tasks sequentially and stop if any task returns "fail".
    pub fn execute_tasks(&mut self) -> Vec<String> {
        todo!()
    }

    // Restart the scheduler by resetting shared state and re-running all tasks.
    pub fn restart(&mut self) -> Vec<String> {
        todo!()
    }

    // Get the shared state value.
    pub fn get_state(&self) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::TaskScheduler;

    #[test]
    fn test_sequential_execution() {
        let mut scheduler = TaskScheduler::new();

        scheduler.register_task(|| "Task 1".to_string());
        scheduler.register_task(|| "Task 2".to_string());
        scheduler.register_task(|| "Task 3".to_string());

        let ret = scheduler.execute_tasks();
        assert_eq!(ret, vec!["Task 1", "Task 2", "Task 3"]);
    }

    #[test]
    fn test_stop_on_failure() {
        let mut scheduler = TaskScheduler::new();

        scheduler.register_task(|| "Task 1".to_string());
        scheduler.register_task(|| "fail".to_string());
        scheduler.register_task(|| "Task 3".to_string());

        let ret = scheduler.execute_tasks();
        assert_eq!(ret, vec!["Task 1", "fail"]);
    }

    #[test]
    fn test_restart_with_state_reset() {
        let mut scheduler = TaskScheduler::new();

        scheduler.register_task({
            let state = scheduler.shared_state.clone();
            move || {
                *state.borrow_mut() += 1;
                format!("State incremented to {}", state.borrow())
            }
        });

        scheduler.execute_tasks();
        assert_eq!(scheduler.get_state(), 1);

        let ret = scheduler.restart();
        assert_eq!(ret, vec!["State incremented to 1"]);
        assert_eq!(scheduler.get_state(), 1);
    }

    #[test]
    fn test_shared_state_across_tasks() {
        let mut scheduler = TaskScheduler::new();

        scheduler.register_task({
            let state = scheduler.shared_state.clone();
            move || {
                *state.borrow_mut() += 1;
                "Task 1 executed".to_string()
            }
        });

        scheduler.register_task({
            let state = scheduler.shared_state.clone();
            move || {
                *state.borrow_mut() += 1;
                "Task 2 executed".to_string()
            }
        });

        scheduler.execute_tasks();
        assert_eq!(scheduler.get_state(), 2);
    }
}
