
use super::{RgbColor, ValueId, Length};
use std::str;

pub use svgparser::AttributeId;
pub use svgparser::AttributeValue as RefAttributeValue;

pub struct Attribute {
    pub id: AttributeId,
    pub value: AttributeValue,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AttributeValue {
    None,
    Number(f64),
    Length(Length),
    RgbColor(RgbColor),
    String(String),
    KeyWord(ValueId),
    Unsupported,
}

impl AttributeValue {
    pub fn from_ref(val: RefAttributeValue) -> AttributeValue {
        match val {
            RefAttributeValue::PredefValue(v) => { AttributeValue::KeyWord(v) }
            RefAttributeValue::Color(c) => { AttributeValue::RgbColor(c) }
            RefAttributeValue::Length(l) => { AttributeValue::Length(l) }
            RefAttributeValue::Number(n) => { AttributeValue::Number(n as f64) }
            RefAttributeValue::String(s) => {
                AttributeValue::String(unsafe { str::from_utf8_unchecked(s).to_string() })
            }
            _ => {
                 // TODO...
                println!(" -- WARNING: unsupported attribute value {:?}", val);
                AttributeValue::Unsupported
            }
        }
    }
}
