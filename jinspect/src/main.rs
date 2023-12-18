use clap::{Arg, ArgAction, Command};
use std::fs::File;
use std::io::BufReader;

mod flag;
mod pool;
mod reader;
use crate::flag::AccessFlag;
use crate::pool::{ConstantPool, ConstantPoolItem};
use crate::reader::{read_n, read_u16, read_u32};

struct Header(u32, u16, u16);

impl Header {
    fn from(rdr: &mut BufReader<File>) -> Self {
        let magic = read_u32(rdr);
        let minor = read_u16(rdr);
        let major = read_u16(rdr);
        Header(magic, major, minor)
    }

    fn print(&self) {
        println!("INFO: Header");
        println!(
            "    Magic= 0x{:X}, Major= {}, Minor= {}",
            self.0, self.1, self.2
        );
    }
}

fn parse_interfaces(reader: &mut BufReader<File>, pool: &ConstantPool) -> Vec<String> {
    let count = read_u16(reader);
    let mut acc = Vec::<String>::with_capacity(count as usize);
    for _ in 0..count {
        let index = read_u16(reader);
        acc.push(pool.resolve(index));
    }
    acc
}

struct RawAttribute(String, u32, Vec<u8>);
fn parse_attributes(
    reader: &mut BufReader<File>,
    pool: &ConstantPool,
) -> Option<Vec<RawAttribute>> {
    let count = read_u16(reader);
    if count > 0 {
        let mut acc = Vec::<RawAttribute>::with_capacity(count as usize);
        for _i in 0..count {
            let name_index = read_u16(reader);
            let name = pool.resolve(name_index);
            let length = read_u32(reader);
            let val = read_n(reader, length as usize);
            acc.push(RawAttribute(name, length, val));
        }
        Some(acc)
    } else {
        None
    }
}

struct Field(AccessFlag, u16, u16, Option<Vec<RawAttribute>>);
fn parse_fields(reader: &mut BufReader<File>, pool: &ConstantPool) -> Option<Vec<Field>> {
    let count = read_u16(reader);
    if count > 0 {
        let mut acc = Vec::<Field>::with_capacity(count as usize);
        for _ in 0..count {
            let flags = AccessFlag::parse_field_level(reader);
            let name_index = read_u16(reader);
            let desc_index = read_u16(reader);
            let attrs = parse_attributes(reader, pool);
            acc.push(Field(flags, name_index, desc_index, attrs));
        }
        Some(acc)
    } else {
        None
    }
}

struct Method(AccessFlag, u16, u16, Option<Vec<RawAttribute>>);
fn parse_methods(reader: &mut BufReader<File>, pool: &ConstantPool) -> Option<Vec<Method>> {
    let count = read_u16(reader);
    if count > 0 {
        let mut acc = Vec::<Method>::with_capacity(count as usize);
        for _ in 0..count {
            let flags = AccessFlag::parse_method_level(reader);
            let name_index = read_u16(reader);
            let desc_index = read_u16(reader);
            let attrs = parse_attributes(reader, pool);
            acc.push(Method(flags, name_index, desc_index, attrs));
        }
        Some(acc)
    } else {
        None
    }
}

// TODO: should not return an option
fn parse_this_class(reader: &mut BufReader<File>, pool: &ConstantPool) -> Option<ConstantPoolItem> {
    let class_index = read_u16(reader);
    if let &ConstantPoolItem::Class(_) = pool.get(class_index) {
        return Some(ConstantPoolItem::Class(class_index));
    }
    None
}

// TODO: should not return an option
fn parse_super_class(
    reader: &mut BufReader<File>,
    pool: &ConstantPool,
) -> Option<ConstantPoolItem> {
    let class_index = read_u16(reader);
    if let &ConstantPoolItem::Class(_) = pool.get(class_index) {
        return Some(ConstantPoolItem::Class(class_index));
    }
    None
}

fn main() {
    let matches = Command::new("jinspect")
        .version("0.1.0")
        .about("inspects java class files")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .default_value("samples/App.class"),
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

            let header = Header::from(&mut reader);
            let constant_pool = ConstantPool::from(&mut reader);
            let acc_class = AccessFlag::parse_class_level(&mut reader);
            let this_class = parse_this_class(&mut reader, &constant_pool);
            let super_class = parse_super_class(&mut reader, &constant_pool);
            let interfaces = parse_interfaces(&mut reader, &constant_pool);
            let fields = parse_fields(&mut reader, &constant_pool);
            let methods = parse_methods(&mut reader, &constant_pool);
            let attributes = parse_attributes(&mut reader, &constant_pool);

            if verbose {
                header.print();
                constant_pool.print();
                acc_class.print();

                if let Some(this_item) = this_class {
                    println!("INFO: ThisClass= {}", this_item.resolve(&constant_pool));
                    if let Some(super_item) = super_class {
                        println!("INFO: SuperClass= {}", super_item.resolve(&constant_pool));
                    }
                }

                println!("INFO: Interfaces= {}", interfaces.capacity());
                for (i, item) in interfaces.iter().enumerate() {
                    println!("    {:02} - {}", i, item);
                }

                match &fields {
                    Some(items) => {
                        println!("INFO: Fields= {}", items.capacity());
                        for (i, item) in items.iter().enumerate() {
                            let flag = item.0.to_string();
                            let name = constant_pool.resolve(item.1);
                            let desc = constant_pool.resolve(item.2);
                            println!("    {:02} {} {} {}", i, flag, name, desc);
                        }
                    }
                    None => println!("INFO: Fields= 0"),
                }
                match &methods {
                    Some(items) => {
                        println!("INFO: Methods= {}", items.capacity());
                        for (i, item) in items.iter().enumerate() {
                            let flag = item.0.to_string();
                            let name = constant_pool.resolve(item.1);
                            let desc = constant_pool.resolve(item.2);
                            println!("    {:02} {} {} {}", i, flag, name, desc);
                        }
                    }
                    None => println!("INFO: Methods= 0"),
                }
                match &attributes {
                    Some(items) => {
                        println!("INFO: Attributes= {}", items.capacity());
                        for (i, item) in items.iter().enumerate() {
                            let name = item.0.to_string();
                            let len = item.1;
                            println!("    {:02} {} {}", i, name, len);
                        }
                    }
                    None => println!("INFO: Attributes= 0"),
                }
            }
        }
        Err(e) => {
            eprintln!("ERROR: could not open file: {file_path}: {e}");
        }
    }
}
