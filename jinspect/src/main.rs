use std::fs::File;
use std::io::BufReader;

mod cli;
mod flag;
mod pool;
mod reader;
use crate::flag::AccessFlag;
use crate::pool::{ConstantPool, ConstantPoolItem};
use crate::reader::{read_u16, read_u32};

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

fn parse_interfaces(reader: &mut BufReader<File>, pool: &ConstantPool) -> Option<Vec<String>> {
    let count = read_u16(reader);
    if count > 0 {
        let mut acc = Vec::<String>::with_capacity(count as usize);
        for _ in 0..count {
            let index = read_u16(reader);
            acc.push(pool.resolve(index));
        }
        return Some(acc);
    }
    None
}

struct RawAttribute(String, u32);
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
            // let val = read_n(reader, length as usize);
            acc.push(RawAttribute(name, length));
        }
        Some(acc)
    } else {
        None
    }
}

struct Field(AccessFlag, String, String, Option<Vec<RawAttribute>>);
fn parse_fields(reader: &mut BufReader<File>, pool: &ConstantPool) -> Option<Vec<Field>> {
    let count = read_u16(reader);
    if count > 0 {
        let mut acc = Vec::<Field>::with_capacity(count as usize);
        for _ in 0..count {
            let flags = AccessFlag::parse_field_level(reader);
            let name_index = read_u16(reader);
            let desc_index = read_u16(reader);
            let name = pool.resolve(name_index);
            let desc = pool.resolve(desc_index);
            let attrs = parse_attributes(reader, pool);
            acc.push(Field(flags, name, desc, attrs));
        }
        Some(acc)
    } else {
        None
    }
}

struct Method(AccessFlag, String, String, Option<Vec<RawAttribute>>);
fn parse_methods(reader: &mut BufReader<File>, pool: &ConstantPool) -> Option<Vec<Method>> {
    let count = read_u16(reader);
    if count > 0 {
        let mut acc = Vec::<Method>::with_capacity(count as usize);
        for _ in 0..count {
            let flags = AccessFlag::parse_method_level(reader);
            let name_index = read_u16(reader);
            let desc_index = read_u16(reader);
            let name = pool.resolve(name_index);
            let desc = pool.resolve(desc_index);
            let attrs = parse_attributes(reader, pool);
            acc.push(Method(flags, name, desc, attrs));
        }
        Some(acc)
    } else {
        None
    }
}

fn parse_this_class(reader: &mut BufReader<File>, pool: &ConstantPool) -> Option<ConstantPoolItem> {
    let class_index = read_u16(reader);
    if let &ConstantPoolItem::Class(_) = pool.get(class_index) {
        return Some(ConstantPoolItem::Class(class_index));
    }
    None
}

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
    let (file_path, verbose) = cli::parse_cli_args();
    match File::open(&file_path) {
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

            if verbose.can_verbose_header() {
                header.print();
            }

            if verbose.can_verbose_class() {
                acc_class.print();
                if let Some(this_item) = this_class {
                    println!("INFO: ThisClass= {}", this_item.resolve(&constant_pool));
                    if let Some(super_item) = super_class {
                        println!("INFO: SuperClass= {}", super_item.resolve(&constant_pool));
                    }
                }
            }

            if verbose.can_verbose_pool() {
                constant_pool.print();
            }

            if verbose.can_verbose_interfaces() {
                match interfaces {
                    Some(items) => {
                        println!("INFO: Interfaces= {}", items.capacity());
                        for (i, item) in items.iter().enumerate() {
                            println!("    {:02} {} ", i, item);
                        }
                    }
                    None => println!("INFO: Interfaces= 0"),
                }
            }

            if verbose.can_verbose_fields() {
                match fields {
                    Some(items) => {
                        println!("INFO: Fields= {}", items.capacity());
                        for (i, item) in items.iter().enumerate() {
                            let flag = item.0.to_string();
                            println!("    {:02} {} {} {}", i, flag, item.1, item.2);
                            let attributes = &item.3;
                            match attributes {
                                Some(items) => {
                                    println!("      Field Attributes= {}", items.capacity());
                                    for (i, item) in items.iter().enumerate() {
                                        let name = item.0.to_string();
                                        let len = item.1;
                                        println!("        {:02} {} {}", i, name, len);
                                    }
                                }

                                None => println!("      Field Attributes= 0"),
                            }
                        }
                    }
                    None => println!("INFO: Fields= 0"),
                }
            }

            if verbose.can_verbose_methods() {
                match methods {
                    Some(items) => {
                        println!("INFO: Methods= {}", items.capacity());
                        for (i, item) in items.iter().enumerate() {
                            let flag = item.0.to_string();
                            println!("    {:02} {} {} {}", i, flag, item.1, item.2);
                            let attributes = &item.3;
                            match attributes {
                                Some(items) => {
                                    println!("      Method Attributes= {}", items.capacity());
                                    for (i, item) in items.iter().enumerate() {
                                        let name = item.0.to_string();
                                        let len = item.1;
                                        println!("        {:02} {} {}", i, name, len);
                                    }
                                }

                                None => println!("      Method Attributes= 0"),
                            }
                        }
                    }
                    None => println!("INFO: Methods= 0"),
                }
            }
            if verbose.can_verbose_attributes() {
                match attributes {
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
