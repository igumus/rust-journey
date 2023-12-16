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

fn parse_attributes(reader: &mut BufReader<File>, pool: &ConstantPool, internal: bool) {
    let count = read_u16(reader);
    if !internal {
        println!("INFO: Attributes= {}", count);
    }
    for i in 0..count {
        let name_index = read_u16(reader);
        let name = pool.resolve(name_index);
        let length = read_u32(reader);
        if name == "Code" {
            let _max_stack = read_u16(reader);
            let _max_locals = read_u16(reader);
            let code_length = read_u32(reader);
            let _code = read_n(reader, code_length as usize);
            let exception_table_len = read_u16(reader);
            for _ in 0..exception_table_len {
                let _start_pc = read_u16(reader);
                let _end_pc = read_u16(reader);
                let _handler_pc = read_u16(reader);
                let _catch_type = read_u16(reader);
            }

            // println!( "MaxStack: {}, MaxLocals: {}, CodeLen: {}, Array: {:?}", max_stack, max_locals, code_length, code);
            parse_attributes(reader, pool, true);
        } else if name == "LineNumberTable" {
            let lnt_length = read_u16(reader);
            for _ in 0..lnt_length {
                let _start_pc = read_u16(reader);
                let _line_number = read_u16(reader);
                // println!( "LineNumberTable= start: {}, lineNumber: {}", start_pc, line_number);
            }
        } else if name == "SourceFile" {
            let source_file_index = read_u16(reader);
            let source_file_name = pool.resolve(source_file_index);
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

fn parse_fields(reader: &mut BufReader<File>, pool: &ConstantPool, debug: bool) {
    let count = read_u16(reader);
    if debug {
        println!("INFO: Fields= {}", count);
    }
    for i in 0..count {
        let flags = AccessFlag::parse_field_level(reader);
        let name_index = read_u16(reader);
        let desc_index = read_u16(reader);
        if debug {
            println!(
                "    Field: {:02} - AF: {}, NI: {} DI: {}",
                i,
                flags.to_string(),
                name_index,
                desc_index
            );
        }
        parse_attributes(reader, pool, true);
    }
}

fn parse_methods(reader: &mut BufReader<File>, pool: &ConstantPool) {
    let count = read_u16(reader);
    println!("INFO: Methods= {}", count);
    for i in 0..count {
        let flags = AccessFlag::parse_method_level(reader);
        let name_index = read_u16(reader);
        let name = pool.resolve(name_index);
        let desc_index = read_u16(reader);
        let desc = pool.resolve(desc_index);
        println!(
            "    Method: {:02} - AF: {}, Name: {} DI: {}",
            i,
            flags.to_string(),
            name,
            desc
        );
        parse_attributes(reader, pool, true);
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

            let header = Header::from(&mut reader);
            let constant_pool = ConstantPool::from(&mut reader);
            let acc_class = AccessFlag::parse_class_level(&mut reader);
            let this_class = parse_this_class(&mut reader, &constant_pool);
            let super_class = parse_super_class(&mut reader, &constant_pool);

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
