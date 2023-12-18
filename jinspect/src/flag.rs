use core::slice::Iter;
use std::fs::File;
use std::io::BufReader;

use crate::reader;

const ACC_CLASS: [(&str, u16); 8] = [
    ("public", 0x0001),
    ("final", 0x0010),
    ("super", 0x0020),
    ("interface", 0x0200),
    ("abstract", 0x0400),
    ("synthetic", 0x1000),
    ("annotation", 0x2000),
    ("enum", 0x4000),
];

const ACC_FIELD: [(&str, u16); 9] = [
    ("public", 0x0001),
    ("private", 0x0002),
    ("protected", 0x0004),
    ("static", 0x0008),
    ("final", 0x0010),
    ("volatile", 0x0040),
    ("transient", 0x0080),
    ("synthetic", 0x1000),
    ("enum", 0x4000),
];

const ACC_METHOD: [(&str, u16); 12] = [
    ("public", 0x0001),
    ("private", 0x0002),
    ("protected", 0x0004),
    ("static", 0x0008),
    ("final", 0x0010),
    ("synchronized", 0x0020),
    ("bridge", 0x0040),
    ("varargs", 0x0080),
    ("native", 0x0100),
    ("abstract", 0x0400),
    ("strict", 0x0800),
    ("synthetic", 0x1000),
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
        let ret: Vec<String> = coll
            .filter(|(_, mask)| item & *mask == *mask)
            .map(|(caption, _)| caption.to_string())
            .collect();
        ret.join(" ")
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
