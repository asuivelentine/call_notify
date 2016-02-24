//!share notification about incomming calls between your PC and your android 
//!phone
#[deny(missing_docs,
       non_camel_case_types,
       non_snake_case,
       unused_import_braces,
       unsafe_code)]

extern crate service_discovery;
extern crate rustc_serialize;

mod net_sd;
mod net_tcp;
pub mod con;
mod msg;
mod msg_handler;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
