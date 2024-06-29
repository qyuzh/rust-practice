use std::any::Any;

pub enum ObjectType {
    Integer,
    Boolean,
    Null,
}

pub trait Object {
    fn inspect(&self) -> String;
    fn typ(&self) -> ObjectType;
    fn as_any(&self) -> &dyn Any;
}

pub struct Integer {
    pub(crate) value: i64,
}

impl Object for Integer {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }

    fn typ(&self) -> ObjectType {
        ObjectType::Integer
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Boolean {
    pub(crate) value: bool,
}

impl Object for Boolean {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }

    fn typ(&self) -> ObjectType {
        ObjectType::Boolean
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Null;

impl Object for Null {
    fn inspect(&self) -> String {
        "null".into()
    }

    fn typ(&self) -> ObjectType {
        ObjectType::Null
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
