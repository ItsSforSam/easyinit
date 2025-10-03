//! General purpose functions

use std::sync::atomic::AtomicU8;


static SHUTING_DOWN: AtomicU8 = AtomicU8::new(0);

pub fn shutdown()->!{
    todo!()

}