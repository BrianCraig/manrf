#![feature(fn_traits)]

use std::rc::Rc;

fn main() {}

pub trait Leaf {
    fn to_string(&self) -> String;
}

pub struct Text {
    val: &'static str,
}

impl Text {
    pub fn new(val: &'static str) -> Rc<Self> {
        Rc::new(Self { val })
    }
}

impl Leaf for Text {
    fn to_string(&self) -> String {
        self.val.to_string()
    }
}

pub struct Number {
    val: i32,
}

impl Number {
    pub fn new(val: i32) -> Rc<Self> {
        Rc::new(Self { val })
    }
}

impl Leaf for Number {
    fn to_string(&self) -> String {
        self.val.to_string()
    }
}



type Component<T> = fn(T) -> Rc<dyn Leaf>;

trait Runner {
    fn to_string(&self) -> String;
}

pub struct App<T> where T: Copy {
    root: Component<T>,
    defaults: T
}

impl<T> App<T> where T: Copy {
    pub fn new(root: Component<T>, defaults: T) -> Self {
        Self { root, defaults }
    }
}

impl<T> Runner for App<T> where T: Copy  {
    fn to_string(&self) -> String {
        let t = (self.defaults,);
        let r = self.root.call(t);
        r.to_string()
    }
}



#[cfg(test)]
pub mod app_tests;
