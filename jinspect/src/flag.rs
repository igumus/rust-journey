use core::slice::Iter;
use std::fs::File;
use std::io::BufReader;

use crate::reader;

const ACC_CLASS: [(&str, u16); 8] = [
    ("Public", 0x0001),
    ("Final", 0x0010),
    ("Super", 0x0020),
    ("Interface", 0x0200),
    ("Abstract", 0x0400),
    ("Synthetic", 0x1000),
    ("Annotation", 0x2000),
    ("Enum", 0x4000),
];

const ACC_FIELD: [(&str, u16); 9] = [
    ("Public", 0x0001),
    ("Private", 0x0002),
    ("Protected", 0x0004),
    ("Static", 0x0008),
    ("Final", 0x0010),
    ("Volatile", 0x0040),
    ("Transient", 0x0080),
    ("Synthetic", 0x1000),
    ("Enum", 0x4000),
];

const ACC_METHOD: [(&str, u16); 12] = [
    ("Public", 0x0001),
    ("Private", 0x0002),
    ("Protected", 0x0004),
    ("Static", 0x0008),
    ("Final", 0x0010),
    ("Synchronized", 0x0020),
    ("Bridge", 0x0040),
    ("Varargs", 0x0080),
    ("Native", 0x0100),
    ("Abstract", 0x0400),
    ("Strict", 0x0800),
    ("Synthetic", 0x1000),
];

pub enum AccessFlag {
    ClassLevel(u16),
    FieldLevel(u16),
    MethodLevel(u16),
}

impl AccessFlag {
    fn parse(reader: &mut BufReader<File>) -> u16 {
        reader::read_u16(reader)
    }

    pub fn parse_class_level(reader: &mut BufReader<File>) -> Self {
        Self::ClassLevel(Self::parse(reader))
    }

    pub fn parse_field_level(reader: &mut BufReader<File>) -> Self {
        Self::FieldLevel(Self::parse(reader))
    }

    pub fn parse_method_level(reader: &mut BufReader<File>) -> Self {
        Self::MethodLevel(Self::parse(reader))
    }

    fn collect_to_string(&self, item: u16, coll: Iter<(&str, u16)>) -> String {
        let mut acc = Vec::<String>::new();
        for (caption, mask) in coll {
            if item & *mask == *mask {
                acc.push(caption.to_string())
            }
        }
        acc.join(";")
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::ClassLevel(item) => self.collect_to_string(*item, ACC_CLASS.iter()),
            Self::FieldLevel(item) => self.collect_to_string(*item, ACC_FIELD.iter()),
            Self::MethodLevel(item) => self.collect_to_string(*item, ACC_METHOD.iter()),
        }
    }

    pub fn print(&self) {
        let level = match self {
            Self::ClassLevel(_) => "Class",
            Self::FieldLevel(_) => "Field",
            Self::MethodLevel(_) => "Method",
        };
        println!("INFO: AccessFlag= {} => {}", level, self.to_string())
    }
}
