use std::fmt;
use std::fs::File;
use std::io::BufReader;

use crate::reader;

const CONSTANTPOOL_UTF8: u8 = 1;
const CONSTANTPOOL_INTEGER: u8 = 3;
const CONSTANTPOOL_FLOAT: u8 = 4;
const CONSTANTPOOL_LONG: u8 = 5;
const CONSTANTPOOL_DOUBLE: u8 = 6;
const CONSTANTPOOL_CLASS: u8 = 7;
const CONSTANTPOOL_STRING: u8 = 8;
const CONSTANTPOOL_FIELDREF: u8 = 9;
const CONSTANTPOOL_METHODREF: u8 = 10;
const CONSTANTPOOL_INTERFACEMETHODREF: u8 = 11;
const CONSTANTPOOL_NAMEANDTYPE: u8 = 12;
const CONSTANTPOOL_METHODHANDLE: u8 = 15;
const CONSTANTPOOL_METHODTYPE: u8 = 16;
const CONSTANTPOOL_INVOKEDYNAMIC: u8 = 18;

#[derive(Debug)]
pub enum ConstantPoolItem {
    Unknown(u8),
    Utf8(String),              // bytes/content
    Class(u16),                // nameIndex
    String(u16),               // stringIndex
    Field(u16, u16),           // classIndex, nameAndTypeIndex
    Method(u16, u16),          // classIndex, nameAndTypeIndex
    NameAndType(u16, u16),     // nameIndex, descriptorIndex
    InterfaceMethod(u16, u16), // classIndex, nameAndTypeIndex
    MethodHandle(u8, u16),     // referenceKind, referenceIndex
    MethodType(u16),           // descriptorIndex
    InvokeDynamic(u16, u16),   // bootstrapMethodAttrIndex, nameAndTypeIndex
    Integer(u32),              // bytes/content
    Float(u32),                // bytes/content
    Long(u32, u32),            // high, low bytes
    Double(u32, u32),          // high, low bytes
}

impl fmt::Display for ConstantPoolItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Utf8(content) => write!(f, "UTF8 => Value: {}", content),
            Self::Class(index) => write!(f, "Class => Index: {}", index),
            Self::String(index) => write!(f, "String => Index: {}", index),
            Self::Field(class, nat) => {
                write!(f, "Field => ClassIndex: {}, NatIndex: {}", class, nat)
            }
            Self::Method(class, nat) => {
                write!(f, "Method => ClassIndex: {}, NatIndex: {}", class, nat)
            }
            Self::NameAndType(name, desc) => {
                write!(f, "NameAndType => NameIndex: {}, DescIndex: {}", name, desc)
            }
            Self::InterfaceMethod(class, nat) => write!(
                f,
                "InterfaceMethod => ClassIndex: {}, NatIndex: {}",
                class, nat
            ),
            Self::MethodHandle(kind, index) => {
                write!(f, "MethodHandle => Kind: {}, RefIndex: {}", kind, index)
            }
            Self::MethodType(index) => write!(f, "MethodType => DescIndex: {}", index),
            Self::InvokeDynamic(class_index, nat_index) => write!(
                f,
                "InvokeDynamic => BootstrapMethodAttrIndex: {}, NatIndex: {}",
                class_index, nat_index
            ),
            Self::Integer(index) => write!(f, "Integer => Value: {}", index),
            Self::Float(index) => write!(f, "Float => Value: {}", index),
            Self::Long(high, low) => write!(f, "Long => High: {}, Low: {}", high, low),
            Self::Double(high, low) => write!(f, "Double => High: {}, Low: {}", high, low),
            Self::Unknown(tag) => write!(f, "Unknown => Tag: {}", tag),
        }
    }
}

impl ConstantPoolItem {
    pub fn resolve(&self, pool: &ConstantPool) -> String {
        match self {
            Self::Utf8(c) => c.to_string(),
            Self::Unknown(c) => format!("Unknown({})", c),
            Self::Integer(c) => format!("{}", c),
            Self::Float(c) => format!("{}", c),
            Self::Long(c1, c2) => format!("H:{},L:{}", c1, c2),
            Self::Double(c1, c2) => format!("H:{},L:{}", c1, c2),
            Self::Class(index) | Self::String(index) | Self::MethodType(index) => {
                let item = pool.get(index.clone());
                item.resolve(pool)
            }
            Self::NameAndType(name_index, type_index) => {
                let n = pool.get(name_index.clone());
                let t = pool.get(type_index.clone());
                format!("{} {}", n.resolve(pool), t.resolve(pool))
            }
            Self::Field(name_index, type_index) => {
                let n = pool.get(name_index.clone());
                let t = pool.get(type_index.clone());
                format!("Field: {} {}", n.resolve(pool), t.resolve(pool))
            }
            _ => "not implemented yet".to_string(),
        }
    }

    pub fn as_type(&self) -> String {
        match self {
            Self::Utf8(_) => return "UTF8".to_string(),
            Self::Class(_) => return "CLASS".to_string(),
            Self::String(_) => return "STRING".to_string(),
            Self::Field(_, _) => return "FIELD".to_string(),
            Self::Method(_, _) => return "METHOD".to_string(),
            Self::NameAndType(_, _) => return "NAME_AND_TYPE".to_string(),
            Self::InterfaceMethod(_, _) => return "INTERFACE_METHOD".to_string(),
            Self::MethodHandle(_, _) => return "METHOD_HANDLE".to_string(),
            Self::MethodType(_) => return "METHOD_TYPE".to_string(),
            Self::InvokeDynamic(_, _) => return "INVOKE_DYNAMIC".to_string(),
            Self::Integer(_) => return "INTEGER".to_string(),
            Self::Float(_) => return "FLOAT".to_string(),
            Self::Long(_, _) => return "LONG".to_string(),
            Self::Double(_, _) => return "DOUBLE".to_string(),
            Self::Unknown(_) => return "UNKNOWN".to_string(),
        }
    }

    pub fn as_value(&self) -> String {
        match self {
            Self::Utf8(content) => content.clone(),
            _ => self.as_type(),
        }
    }
}

pub struct ConstantPool(Vec<ConstantPoolItem>);

impl ConstantPool {
    pub fn from(rdr: &mut BufReader<File>) -> Self {
        let constant_pool_count = reader::read_u16(rdr);
        let mut ret = Vec::<ConstantPoolItem>::with_capacity(constant_pool_count as usize);
        for _ in 1..(constant_pool_count) {
            let tag = reader::read_u8(rdr);
            let item = match tag {
                CONSTANTPOOL_CLASS => {
                    let index = reader::read_u16(rdr);
                    ConstantPoolItem::Class(index)
                }
                CONSTANTPOOL_METHODREF => {
                    let class = reader::read_u16(rdr);
                    let nat = reader::read_u16(rdr);
                    ConstantPoolItem::Method(class, nat)
                }
                CONSTANTPOOL_NAMEANDTYPE => {
                    let class = reader::read_u16(rdr);
                    let nat = reader::read_u16(rdr);
                    ConstantPoolItem::NameAndType(class, nat)
                }
                CONSTANTPOOL_UTF8 => {
                    let length = reader::read_u16(rdr);
                    let value: String = reader::read_str(rdr, length as usize);
                    ConstantPoolItem::Utf8(value)
                }
                CONSTANTPOOL_FIELDREF => {
                    let class = reader::read_u16(rdr);
                    let nat = reader::read_u16(rdr);
                    ConstantPoolItem::Field(class, nat)
                }
                CONSTANTPOOL_INTERFACEMETHODREF => {
                    let class = reader::read_u16(rdr);
                    let nat = reader::read_u16(rdr);
                    ConstantPoolItem::InterfaceMethod(class, nat)
                }
                CONSTANTPOOL_STRING => {
                    let class_index = reader::read_u16(rdr);
                    ConstantPoolItem::String(class_index)
                }
                CONSTANTPOOL_INTEGER => {
                    let val = reader::read_u32(rdr);
                    ConstantPoolItem::Integer(val)
                }
                CONSTANTPOOL_FLOAT => {
                    let val = reader::read_u32(rdr);
                    ConstantPoolItem::Float(val)
                }
                CONSTANTPOOL_LONG => {
                    let high_val = reader::read_u32(rdr);
                    let low_val = reader::read_u32(rdr);
                    ConstantPoolItem::Long(high_val, low_val)
                }
                CONSTANTPOOL_DOUBLE => {
                    let high_val = reader::read_u32(rdr);
                    let low_val = reader::read_u32(rdr);
                    ConstantPoolItem::Double(high_val, low_val)
                }
                CONSTANTPOOL_METHODHANDLE => {
                    let kind = reader::read_u8(rdr);
                    let index = reader::read_u16(rdr);
                    ConstantPoolItem::MethodHandle(kind, index)
                }
                CONSTANTPOOL_METHODTYPE => {
                    let index = reader::read_u16(rdr);
                    ConstantPoolItem::MethodType(index)
                }
                CONSTANTPOOL_INVOKEDYNAMIC => {
                    let attr_index = reader::read_u16(rdr);
                    let name_and_type_index = reader::read_u16(rdr);
                    ConstantPoolItem::InvokeDynamic(attr_index, name_and_type_index)
                }
                _ => ConstantPoolItem::Unknown(tag),
            };
            ret.push(item);
        }
        ConstantPool(ret)
    }

    pub fn print(&self) {
        println!("INFO: ConstantPool= {:02}", self.0.capacity());
        for (i, item) in self.0.iter().enumerate() {
            println!("    {:03} {}", (i + 1), item);
        }
    }

    pub fn get(&self, index: u16) -> &ConstantPoolItem {
        &self.0[(index - 1) as usize]
    }
}
