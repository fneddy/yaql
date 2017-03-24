use std::convert::From;
use libc::c_void;

use core::QMetaTypeId;

cpp!{{
    #include <iostream>
    #include <QVariant>
}}

pub struct QVariant {
    raw: *mut c_void,
}
impl QVariant{
    pub fn new() -> QVariant{
        let p = unsafe { cpp! ([] -> *mut c_void as "void *"{
            return new QVariant();
        }) };
        QVariant {raw: p}
    }

    pub fn qMetaTypeId(&self) -> QMetaTypeId {
        let p = self.raw;
        QMetaTypeId::from( self.typeId() )
    }
    pub fn typeId(&self) -> i32 {
        let p = self.raw;
        unsafe { cpp! ([ p as "QVariant*"] -> i32 as "QVariant::Type" {
            return p->type();
        })}
    }

}
impl Drop for QVariant {
    fn drop(&mut self) {
        let p = self.raw;
        unsafe {cpp! ([ p as "QVariant*"] {
            delete p;
        })};
    }
}

impl From<i32> for QVariant {
    fn from(v: i32) -> QVariant {
        let a = unsafe { cpp! ([v as "int32_t"] -> *mut c_void as "void *"{
            return new QVariant(v);
        }) };
        QVariant { raw: a}
    }
}

#[test]
fn newQVariant() {
    let v = QVariant::new();
}
#[test]
fn fromi32() {
    let v = QVariant::from(23);
    assert! (v.qMetaTypeId() == QMetaTypeId::Int);
}

