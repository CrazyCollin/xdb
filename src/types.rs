#[derive(Eq, PartialEq, Clone, Debug)]
pub enum TypeClassification{
    Internal,
    UserDefined,
}

impl TypeClassification {
    fn from_byte(val:u8)->Self{
        match val {
            1=>Self::Internal,
            2=>Self::UserDefined,
            _ => unreachable!()
        }
    }

    fn to_byte(&self)->u8{
        match self {
            TypeClassification::Internal=>1,
            TypeClassification::UserDefined=>2,
        }
    }
}

#[derive(Debug,Clone,Eq, PartialEq)]
pub struct  TypeName{
    classification:TypeClassification,
    name:String,
}

impl TypeName {
    pub fn new(name:&str)->Self{
        Self{
            classification:TypeClassification::UserDefined,
            name:name.into(),
        }
    }

    pub fn interval(name:&str)->Self{
        Self{
            classification:TypeClassification::Internal,
            name:name.into(),
        }
    }

    pub fn to_bytes(&self)->Vec<u8>{
        let mut  res=Vec::with_capacity(1+self.name.as_bytes().len());
        res.push(self.classification.to_byte());
        res.extend_from_slice(self.name.as_bytes());
        res
    }

    pub fn from_bytes(bytes:&[u8])->Self{
        let type_name=
        Self{

        }
    }
}



pub trait XDBValue{
    type
}

pub trait XDBKey:XDBValue{

}