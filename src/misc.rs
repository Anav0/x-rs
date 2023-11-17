use std::{
    env::Args,
};

pub struct Parameters {
    pub file_path: String
}

impl From<Args> for Parameters {
    fn from(args: Args) -> Self {
        let collected_args: Vec<String> = args.collect();

        if collected_args.len() < 2 {
            panic!("Compiler needs at least one argument");
        }

        Self {
            file_path: collected_args[1].clone(),
        }
    }
}
