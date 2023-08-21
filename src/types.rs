use std::cmp::Ordering;
use std::fmt::Debug;
use std::str::from_utf8;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum TypeClassification {
    Internal,
    UserDefined,
}

impl TypeClassification {
    fn from_byte(val: u8) -> Self {
        match val {
            1 => Self::Internal,
            2 => Self::UserDefined,
            _ => unreachable!(),
        }
    }

    fn to_byte(&self) -> u8 {
        match self {
            TypeClassification::Internal => 1,
            TypeClassification::UserDefined => 2,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeName {
    classification: TypeClassification,
    name: String,
}

impl TypeName {
    pub fn new(name: &str) -> Self {
        Self {
            classification: TypeClassification::UserDefined,
            name: name.into(),
        }
    }

    pub fn interval(name: &str) -> Self {
        Self {
            classification: TypeClassification::Internal,
            name: name.into(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(1 + self.name.as_bytes().len());
        res.push(self.classification.to_byte());
        res.extend_from_slice(self.name.as_bytes());
        res
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let classification = TypeClassification::from_byte(bytes[0]);
        let name = from_utf8(&bytes[1..]).unwrap().into();
        Self {
            classification,
            name,
        }
    }
}

pub trait XDBValue: Debug {
    type SelfType<'a>: Debug + 'a
    where
        Self: 'a;
    type AsBytes<'a>: AsRef<u8> + 'a
    where
        Self: 'a;

    fn fixed_width() -> Option<usize>;

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a;

    fn to_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'a,
        Self: 'b;

    fn type_name() -> String;
}

pub trait XDBKey: XDBValue {
    fn compare(data1: &[u8], data2: &[u8]) -> Ordering;
}
