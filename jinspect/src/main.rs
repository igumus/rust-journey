use clap::{Arg, ArgAction, Command};
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str;

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
enum ConstantPool {
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

impl ConstantPool {
    fn as_type(&self) -> String {
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

    fn as_value(&self) -> String {
        match self {
            Self::Utf8(content) => content.clone(),
            _ => self.as_type(),
        }
    }
}

impl fmt::Display for ConstantPool {
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

fn read_n(r: &mut BufReader<File>, limit: usize) -> Vec<u8> {
    let mut buf = Vec::<u8>::with_capacity(limit);
    for _i in 0..limit {
        buf.push(0);
    }
    let _ = r.read_exact(&mut buf);
    buf
}

fn read_u8(r: &mut BufReader<File>) -> u8 {
    let mut buf: [u8; 1] = [0; 1];
    let _ = r.read_exact(&mut buf);
    u8::from_be_bytes(buf)
}

fn read_u16(r: &mut BufReader<File>) -> u16 {
    let mut buf: [u8; 2] = [0; 2];
    let _ = r.read_exact(&mut buf);
    u16::from_be_bytes(buf)
}

fn read_u32(r: &mut BufReader<File>) -> u32 {
    let mut buf: [u8; 4] = [0; 4];
    let _ = r.read_exact(&mut buf);
    u32::from_be_bytes(buf)
}

fn read_str(r: &mut BufReader<File>, length: usize) -> String {
    let buf = read_n(r, length);
    str::from_utf8(&buf).unwrap().to_string()
}

fn parse_header(reader: &mut BufReader<File>) -> (u32, u16, u16) {
    let magic = read_u32(reader);
    let minor = read_u16(reader);
    let major = read_u16(reader);
    (magic, major, minor)
}

fn parse_constant_pool(reader: &mut BufReader<File>) -> Vec<ConstantPool> {
    let constant_pool_count = read_u16(reader);
    let mut ret = Vec::<ConstantPool>::with_capacity(constant_pool_count as usize);
    for _ in 1..(constant_pool_count) {
        let tag = read_u8(reader);
        let item = match tag {
            CONSTANTPOOL_CLASS => {
                let index = read_u16(reader);
                ConstantPool::Class(index)
            }
            CONSTANTPOOL_METHODREF => {
                let class = read_u16(reader);
                let nat = read_u16(reader);
                ConstantPool::Method(class, nat)
            }
            CONSTANTPOOL_NAMEANDTYPE => {
                let class = read_u16(reader);
                let nat = read_u16(reader);
                ConstantPool::NameAndType(class, nat)
            }
            CONSTANTPOOL_UTF8 => {
                let length = read_u16(reader);
                let value: String = read_str(reader, length as usize);
                ConstantPool::Utf8(value)
            }
            CONSTANTPOOL_FIELDREF => {
                let class = read_u16(reader);
                let nat = read_u16(reader);
                ConstantPool::Field(class, nat)
            }
            CONSTANTPOOL_INTERFACEMETHODREF => {
                let class = read_u16(reader);
                let nat = read_u16(reader);
                ConstantPool::InterfaceMethod(class, nat)
            }
            CONSTANTPOOL_STRING => {
                let class_index = read_u16(reader);
                ConstantPool::String(class_index)
            }
            CONSTANTPOOL_INTEGER => {
                let val = read_u32(reader);
                ConstantPool::Integer(val)
            }
            CONSTANTPOOL_FLOAT => {
                let val = read_u32(reader);
                ConstantPool::Float(val)
            }
            CONSTANTPOOL_LONG => {
                let high_val = read_u32(reader);
                let low_val = read_u32(reader);
                ConstantPool::Long(high_val, low_val)
            }
            CONSTANTPOOL_DOUBLE => {
                let high_val = read_u32(reader);
                let low_val = read_u32(reader);
                ConstantPool::Double(high_val, low_val)
            }
            CONSTANTPOOL_METHODHANDLE => {
                let kind = read_u8(reader);
                let index = read_u16(reader);
                ConstantPool::MethodHandle(kind, index)
            }
            CONSTANTPOOL_METHODTYPE => {
                let index = read_u16(reader);
                ConstantPool::MethodType(index)
            }
            CONSTANTPOOL_INVOKEDYNAMIC => {
                let attr_index = read_u16(reader);
                let name_and_type_index = read_u16(reader);
                ConstantPool::InvokeDynamic(attr_index, name_and_type_index)
            }
            _ => ConstantPool::Unknown(tag),
        };
        ret.push(item);
    }
    ret
}

fn parse_interfaces(reader: &mut BufReader<File>, debug: bool) {
    let interfaces_count = read_u16(reader);
    if debug {
        println!("INFO: Interfaces= {}", interfaces_count);
    }
    for i in 0..interfaces_count {
        let name_index = read_u16(reader);
        if debug {
            print!("    INTERFACE: {:02} - NameIndex: {}", i, name_index);
        }
    }
}

fn parse_class_access_flags(reader: &mut BufReader<File>) -> u16 {
    let access_flags = read_u16(reader);
    access_flags
}

fn parse_attributes(reader: &mut BufReader<File>, pool: &Vec<ConstantPool>, internal: bool) {
    let count = read_u16(reader);
    if !internal {
        println!("INFO: Attributes= {}", count);
    }
    for i in 0..count {
        let name_index = read_u16(reader);
        let constant = pool.get((name_index - 1) as usize).unwrap();

        let name = constant.as_value();
        let length = read_u32(reader);
        if name == "Code" {
            let max_stack = read_u16(reader);
            let max_locals = read_u16(reader);
            let code_length = read_u32(reader);
            let code = read_n(reader, code_length as usize);
            let exception_table_len = read_u16(reader);
            for _ in 0..exception_table_len {
                let _start_pc = read_u16(reader);
                let _end_pc = read_u16(reader);
                let _handler_pc = read_u16(reader);
                let _catch_type = read_u16(reader);
            }

            println!(
                "MaxStack: {}, MaxLocals: {}, CodeLen: {}, Array: {:?}",
                max_stack, max_locals, code_length, code
            );
            parse_attributes(reader, pool, true);
        } else if name == "LineNumberTable" {
            let lnt_length = read_u16(reader);
            for _ in 0..lnt_length {
                let start_pc = read_u16(reader);
                let line_number = read_u16(reader);
                println!(
                    "LineNumberTable= start: {}, lineNumber: {}",
                    start_pc, line_number
                );
            }
        } else if name == "SourceFile" {
            let source_file_index = read_u16(reader);
            let source_file_name = pool
                .get((source_file_index - 1) as usize)
                .unwrap()
                .as_value();
            if !internal {
                println!("    Attribute: {:02} - SourceFile: {}", i, source_file_name);
            } else {
                println!(
                    "        Attribute: {:02} - SourceFile: {}",
                    i, source_file_name
                );
            }
        } else {
            let _val = read_n(reader, length as usize);
            if !internal {
                println!("    Attribute: {:02} - Name: {} LEN:{}", i, name, length);
            } else {
                println!(
                    "        Attribute: {:02} - Name: {} LEN:{}",
                    i, name, length
                );
            }
        }
    }
}

fn parse_fields(reader: &mut BufReader<File>, pool: &Vec<ConstantPool>, debug: bool) {
    let count = read_u16(reader);
    if debug {
        println!("INFO: Fields= {}", count);
    }
    for i in 0..count {
        let access_flags: u16 = read_u16(reader);
        let name_index = read_u16(reader);
        let desc_index = read_u16(reader);
        if debug {
            println!(
                "    Field: {:02} - AF: {}, NI: {} DI: {}",
                i, access_flags, name_index, desc_index
            );
        }
        parse_attributes(reader, pool, true);
    }
}

fn parse_methods(reader: &mut BufReader<File>, pool: &Vec<ConstantPool>) {
    let count = read_u16(reader);
    println!("INFO: Methods= {}", count);
    for i in 0..count {
        let access_flags: u16 = read_u16(reader);
        let name_index = read_u16(reader);
        let constant = pool.get((name_index - 1) as usize).unwrap();
        let name = constant.as_value();
        let desc_index = read_u16(reader);
        println!(
            "    Method: {:02} - AF: {}, Name: {} DI: {}",
            i, access_flags, name, desc_index
        );
        parse_attributes(reader, pool, true);
    }
}

fn main() {
    let matches = Command::new("jinspect")
        .version("0.1.0")
        .about("inspects java class files")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .default_value("App.class"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let file_path = matches.get_one::<String>("file").expect("required");
    let verbose = matches.get_flag("verbose");

    match File::open(file_path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);

            let (magic, major, minor) = parse_header(&mut reader);
            if verbose {
                println!("INFO: Header");
                println!(
                    "    Magic= 0x{:X}, Major= {}, Minor= {}",
                    magic, major, minor
                );
            }
            let constant_pool = parse_constant_pool(&mut reader);
            if verbose {
                println!("INFO: ConstantPool= {:02}", constant_pool.capacity());
                for (i, item) in constant_pool.iter().enumerate() {
                    println!("    {:03} {}", (i + 1), item);
                }
            }

            let class_access_flags = parse_class_access_flags(&mut reader);
            if verbose {
                println!("INFO: ClassAccessFlags= {}", class_access_flags);
                if class_access_flags & 0x0001 == 0x0001 {
                    println!("    - Public");
                }
                if class_access_flags & 0x0010 == 0x0010 {
                    println!("    - Final");
                }
                if class_access_flags & 0x0020 == 0x0020 {
                    println!("    - Super");
                }
                if class_access_flags & 0x0200 == 0x0200 {
                    println!("    - Interface");
                }
                if class_access_flags & 0x0400 == 0x0400 {
                    println!("    - Abstract");
                }
                if class_access_flags & 0x1000 == 0x1000 {
                    println!("    - Synthetic");
                }
                if class_access_flags & 0x2000 == 0x2000 {
                    println!("    - Annotation");
                }
                if class_access_flags & 0x4000 == 0x4000 {
                    println!("    - Enum");
                }
            }
            let this_class = read_u16(&mut reader);
            let super_class = read_u16(&mut reader);
            if verbose {
                println!("INFO: ThisClass= {}", this_class);
                println!("INFO: SuperClass= {}", super_class);
            }

            parse_interfaces(&mut reader, verbose);
            parse_fields(&mut reader, &constant_pool, verbose);
            parse_methods(&mut reader, &constant_pool);
            parse_attributes(&mut reader, &constant_pool, false);
        }
        Err(e) => {
            eprintln!("ERROR: could not open file: {file_path}: {e}");
        }
    }
}
