//!share notification about incomming calls between your PC and your android 
//!phone
#[deny(missing_docs,
       non_camel_case_types,
       non_snake_case,
       unused_import_braces,
       unsafe_code)]

extern crate service_discovery;
extern crate rustc_serialize;

<<<<<<< dc49a2f2caa507af44712aa687fdf48dd0caaa87
mod net_sd;
mod net_tcp;
pub mod con;
=======
pub mod net_sd;
pub mod net_tcp;
mod msg;
mod msg_handler;
>>>>>>> added msg_handler and msg

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
