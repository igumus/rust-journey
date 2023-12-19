use clap::{Arg, ArgAction, Command};

const VERBOSE_ALL: u8 = 0;
const VERBOSE_HEADER: u8 = 1;
const VERBOSE_CLASS: u8 = 2;
const VERBOSE_POOL: u8 = 4;
const VERBOSE_INTERFACES: u8 = 8;
const VERBOSE_METHODS: u8 = 16;
const VERBOSE_FIELDS: u8 = 32;
const VERBOSE_ATTRIBUTES: u8 = 64;

pub struct VerboseMode(u8);

impl VerboseMode {
    fn build(vals: &Vec<&String>) -> Self {
        let mut ret: u8 = VERBOSE_ALL;

        if vals.contains(&&"header".to_string()) {
            ret = ret | VERBOSE_HEADER;
        }

        if vals.contains(&&"clazz".to_string()) {
            ret = ret | VERBOSE_CLASS;
        }
        if vals.contains(&&"pool".to_string()) {
            ret = ret | VERBOSE_POOL;
        }
        if vals.contains(&&"interface".to_string()) {
            ret = ret | VERBOSE_INTERFACES;
        }
        if vals.contains(&&"method".to_string()) {
            ret = ret | VERBOSE_METHODS;
        }
        if vals.contains(&&"field".to_string()) {
            ret = ret | VERBOSE_FIELDS;
        }
        if vals.contains(&&"attribute".to_string()) {
            ret = ret | VERBOSE_ATTRIBUTES;
        }
        Self(ret)
    }

    fn can_verbose(&self, flag: u8) -> bool {
        (self.0 == VERBOSE_ALL) || (self.0 & flag == flag)
    }

    pub fn can_verbose_header(&self) -> bool {
        self.can_verbose(VERBOSE_HEADER)
    }

    pub fn can_verbose_class(&self) -> bool {
        self.can_verbose(VERBOSE_CLASS)
    }

    pub fn can_verbose_pool(&self) -> bool {
        self.can_verbose(VERBOSE_POOL)
    }
    pub fn can_verbose_interfaces(&self) -> bool {
        self.can_verbose(VERBOSE_INTERFACES)
    }

    pub fn can_verbose_methods(&self) -> bool {
        self.can_verbose(VERBOSE_METHODS)
    }

    pub fn can_verbose_fields(&self) -> bool {
        self.can_verbose(VERBOSE_FIELDS)
    }

    pub fn can_verbose_attributes(&self) -> bool {
        self.can_verbose(VERBOSE_ATTRIBUTES)
    }
}

pub fn parse_cli_args() -> (String, VerboseMode) {
    let matches = Command::new("jinspect")
        .version("0.1.0")
        .about("inspects java class files")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .default_missing_value("samples/App.class")
                .default_value("samples/App.class"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .value_parser([
                    "all",
                    "header",
                    "clazz",
                    "pool",
                    "method",
                    "field",
                    "interface",
                    "attribute",
                ])
                .num_args(0..)
                .action(ArgAction::Set)
                .default_value("all")
                .default_missing_value("all")
                .value_delimiter(',')
                .help("Print all information of class file"),
        )
        .get_matches();
    let file_path = matches.get_one::<String>("file").expect("required");
    let verbose_mode = matches
        .get_many::<String>("verbose")
        .unwrap()
        .collect::<Vec<_>>();

    (file_path.to_string(), VerboseMode::build(&verbose_mode))
}
