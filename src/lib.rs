use std::ops::{Add, Div, Mul, Rem, Sub};

use thiserror::Error;

const MAX_FRAMES: usize = 1024;

#[derive(Error, Debug, PartialEq)]
pub enum StackError {
    #[error("stack overflow")]
    StackOverflow,
    #[error("stack underflow")]
    StackUnderflow,
    #[error("unknown operation")]
    UnknownOperation,
}

#[derive(Copy, Clone)]
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

    fn push(&mut self, data: T) -> Result<(), StackError> {
        if self.ptr < MAX_FRAMES {
            self.arr[self.ptr] = data;
            self.ptr += 1;
            Ok(())
        } else {
            Err(StackError::StackOverflow)
        }
    }

    fn pop(&mut self) -> Result<T, StackError> {
        if self.ptr != 0 {
            self.ptr -= 1;
            Ok(self.arr[self.ptr])
        } else {
            Err(StackError::StackUnderflow)
        }
    }

    pub fn execute(&mut self, op: Operation<T>) -> Result<(), StackError> {
        match op {
            Operation::PUSH1(data) => self.push(data),
            _ => {
                let a = self.pop()?;
                let b = self.pop()?;
                let res = match op {
                    Operation::ADD => a + b,
                    Operation::SUB => a - b,
                    Operation::MUL => a * b,
                    // FIXME: How to check for div by 0 with generics ???
                    Operation::DIV => a / b,
                    Operation::MOD => a % b,
                    _ => return Err(StackError::UnknownOperation),
                };
                self.push(res)
            }
        }
    }

    #[cfg(test)]
    fn top(&self) -> T {
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

pub enum Operation<T>
where
    T: Copy + Default,
{
    PUSH1(T),
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
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
        assert_eq!(stack.ptr, MAX_FRAMES - 1);
        // Check pop
        stack.pop().expect("Pop shouldn't fail!");
        assert_eq!(stack.ptr, MAX_FRAMES - 2); // 1022
    }

    #[test]
    fn test_operations() {
        let mut stack = Stack::new();
        stack.execute(Operation::PUSH1(2)).unwrap();
        stack.execute(Operation::PUSH1(10usize)).unwrap();
        let mut stack2 = stack.clone();
        let mut stack3 = stack.clone();
        let mut stack4 = stack.clone();
        let mut stack5 = stack.clone();
        stack.execute(Operation::ADD).unwrap();
        assert_eq!(stack.top(), 12);
        stack2.execute(Operation::SUB).unwrap();
        assert_eq!(stack2.top(), 8);
        stack3.execute(Operation::MUL).unwrap();
        assert_eq!(stack3.top(), 20);
        stack4.execute(Operation::DIV).unwrap();
        assert_eq!(stack4.top(), 5);
        stack5.execute(Operation::MOD).unwrap();
        assert_eq!(stack5.top(), 0);
    }
}
