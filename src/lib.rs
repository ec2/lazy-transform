#![no_std]

extern crate alloc;

use core::cell::UnsafeCell;

enum Value<T, U> {
    Init(T),
    Final(U),
}

pub struct LazyTransform<F, T, U> {
    transform: F,
    value: UnsafeCell<Value<T, U>>,
}

impl<F: Fn(&T) -> U, T, U> LazyTransform<F, T, U> {
    pub fn new<'a>(init: T, f: F) -> LazyTransform<F, T, U> {
        LazyTransform {
            transform: f,
            value: Value::Init(init).into(),
        }
    }
    pub fn get(&self) -> &U {
        let value = unsafe { &mut *self.value.get() };
        match value {
            Value::Init(v) => {
                let final_value = (self.transform)(v);
                *value = Value::Final(final_value);
                match value {
                    Value::Init(_) => unreachable!(),
                    Value::Final(ref final_value) => final_value,
                }
            }
            Value::Final(v) => v,
        }
    }
}

#[cfg(tests)]
mod test {
    fn add_one(x: &i32) -> i32 {
        x + 1
    }
    #[test]
    fn it_works() {
        use crate::LazyTransform;
        let lazy = LazyTransform::new(1, |x| x + 1);
        assert_eq!(*lazy.get(), 2);
        assert_eq!(*lazy.get(), 2);
        let lazy = LazyTransform::new(10, add_one);
        assert_eq!(*lazy.get(), 11);
        assert_eq!(*lazy.get(), 11);
    }
}
