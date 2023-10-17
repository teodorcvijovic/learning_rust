use std::mem::replace;
use core::fmt;
use std::str::FromStr;
use std::error::Error;

#[derive(Debug)]
pub enum List {
    Nil,
    Cons(i32, Box<List>),
}

#[derive(Debug)]
pub struct Stack {
    pub length: u32,
    pub list: List,
}

impl List {
    pub fn len(&self) -> u32 {
        match self {
            List::Nil => 0,
            List::Cons(_, tail) => 1 + tail.len(),
        }
    }
}

impl Stack {
    pub fn new() -> Self {
        Stack{length: 0, list: List::Nil}
    }

    pub fn len(&self) -> u32 {
        self.length
    }

    pub fn push(&mut self, elem: i32) {
        self.list = List::Cons(elem, Box::new(replace(&mut self.list, List::Nil)));
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<i32> {
        match replace(&mut self.list, List::Nil) {
            List::Nil => return None,
            List::Cons(val, new_list) => {
                self.list = *new_list;
                self.length -= 1;
                return Some(val);
            }
        }
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = String::from("[");

        // zanimljivo: iteriranje kroz listu
        let mut curr = &self.list;
        while let List::Cons(val, next) = curr {
            text.push_str(&val.to_string());

            // check if next is Cons or Nil
            if let List::Cons(_, _) = **next {
                text.push_str(", ");
            }

            // deref &Box, then deref Box, take addr of List
            curr = &**next;
        }
        text.push_str("]");

        write!(f, "{text}")
    }
}

impl From<List> for Vec<i32> {
    fn from(list: List) -> Self {
        let mut vec = Vec::new();
        let mut curr = &list;
        while let List::Cons(val, next) = curr {
            vec.push(*val);
            curr = &**next;
        }
        return vec;
    }
}

impl From<Stack> for Vec<i32> {
    fn from(stack: Stack) -> Self {
        Vec::<i32>::from(stack.list)
    }
}

impl From<Vec<i32>> for List {
    fn from(vec: Vec<i32>) -> Self {
        let mut list = List::Nil;
        for val in vec.iter().rev() {
            list = List::Cons(*val, Box::new(list));
        }
        return list;
    }
}

impl From<Vec<i32>> for Stack {
    fn from(vec: Vec<i32>) -> Self {
        let mut stack = Stack::new();
        stack.list = List::from(vec);
        stack.length = stack.list.len();
        stack
    }
}

impl FromStr for List {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut list = List::Nil;

        let mut s = s.trim();
        s = &s[1..(s.len() - 1)];
        let str = s.to_owned();
        let splited = str.split(", ");

        let mut values: Vec<i32> = Vec::new();
        for val in splited {
            if let Ok(num) = val.parse::<i32>() {
                values.push(num);
            }
        }

        for val in values.iter().rev() {
            list = List::Cons(*val, Box::new(list));
        }

        Ok(list)
    }
}

impl FromStr for Stack {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list: List = s.parse()?;
        let mut stack = Stack::new();
        stack.list = list;
        stack.length = stack.list.len();

        Ok(stack)
    }
}

fn main() {
    let l1 = List::Nil;
    let l2 = List::Cons(37, Box::new(l1));
    let l3 = List::Cons(99, Box::new(l2));
    let l4 = List::Cons(12, Box::new(l3));

    println!("list: {l4:?}");

    let mut stack = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(1);
    let _ = stack.pop();
    stack.push(30);
    println!("{stack}");

    let vec = Vec::<i32>::from(stack);
    println!("{vec:?}");
    let stack = Stack::from(vec);
    println!("{stack}");

    let stack: Stack = "[2, 5, 7]".parse().unwrap();
    println!("{stack}");
}
