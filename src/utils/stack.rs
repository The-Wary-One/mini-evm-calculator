use std::ops::{Add, Div, Mul, Rem, Sub};
use thiserror::Error;

pub const MAX_FRAMES: usize = 1024;

#[derive(Error, Debug, PartialEq)]
pub enum StackError {
    #[error("stack overflow")]
    StackOverflow,
    #[error("stack underflow")]
    StackUnderflow,
    #[error("unknown operation")]
    UnknownToken,
}

#[derive(Debug, Copy, Clone)]
pub struct Stack<T> {
    arr: [T; MAX_FRAMES],
    ptr: usize,
}

impl<T> Stack<T>
where
    T: Copy
        + Default
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, data: T) -> Result<(), StackError> {
        if self.ptr < MAX_FRAMES {
            self.arr[self.ptr] = data;
            self.ptr += 1;
            Ok(())
        } else {
            Err(StackError::StackOverflow)
        }
    }

    pub fn pop(&mut self) -> Result<T, StackError> {
        if self.ptr != 0 {
            self.ptr -= 1;
            let top = self.arr[self.ptr];
            self.arr[self.ptr] = T::default();
            Ok(top)
        } else {
            Err(StackError::StackUnderflow)
        }
    }

    pub fn top(&self) -> T {
        self.arr[self.ptr - 1]
    }
}

impl<T> Default for Stack<T>
where
    T: Copy + Default,
{
    fn default() -> Self {
        Stack {
            arr: [T::default(); MAX_FRAMES],
            ptr: usize::default(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        // Check pointer start
        assert_eq!(stack.ptr, 0);
        // Check pop stack underflow protection
        let res = stack.pop();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), StackError::StackUnderflow);
        assert_eq!(stack.ptr, 0);
        // Check push
        let data = 2usize;
        stack.push(data).expect("Push shouldn't fail!");
        assert_eq!(stack.ptr, 1);
        assert_eq!(stack.top(), data);
        // Check push stack overflow protection
        while stack.push(data).is_ok() {}
        assert_eq!(stack.ptr, MAX_FRAMES);
        let res = stack.push(data);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), StackError::StackOverflow);
        assert_eq!(stack.ptr, MAX_FRAMES);
        // Check pop
        stack.pop().expect("Pop shouldn't fail!");
        assert_eq!(stack.ptr, MAX_FRAMES - 1); // 1022
    }
}
