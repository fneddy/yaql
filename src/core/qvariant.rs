use std::convert::From;
use libc::c_void;
use libc::c_char;
use std::char::decode_utf16;
use std::ffi::CString;
use std::cmp::Ordering;

use core::QMetaTypeId;


cpp!{{
    #include <iostream>
    #include <QVariant>
    #include <cstring>
}}

/// QVariant wraps around the Qt/C++ QVariant type
pub struct QVariant {
    raw: *mut c_void,
}
impl QVariant {
    /// use the new method the create an empty QVariant.
    /// if you want to create it from a value use QVariant::from
    pub fn new() -> QVariant {
        let p = unsafe {
            cpp! ([] -> *mut c_void as "void *"{
            return new QVariant();
        })
        };
        QVariant { raw: p }
    }

    /// create a QVariant from a raw C++ QVariant pointer
    pub unsafe fn from_raw(p: *mut c_void) -> QVariant {
        QVariant {raw: p}
    }

    /// get the C++ QVariant pointer. This method will consume self!
    pub unsafe fn into_raw(self) -> *mut c_void {
        self.raw
    }

    /// get the QMetaTypeId of the value. this will not work on QObject values!
    pub fn qmeta_type_id(&self) -> QMetaTypeId {
        QMetaTypeId::from(self.type_id())
    }

    /// get the type id of the value
    pub fn type_id(&self) -> i32 {
        let p = self.raw;
        unsafe {
            cpp! ([ p as "QVariant*"] -> i32 as "QVariant::Type" {
            return p->type();
        })
        }
    }

    /// can this type be converted to this type?
    pub fn can_convert(&self, type_id: i32) -> bool {
        let p = self.raw;
        unsafe {
            cpp!([ p as "QVariant*", type_id as "int32_t"] -> bool as "bool" {
            return p->canConvert(type_id);
        })
        }
    }

    /// clear the value
    pub fn clear(&mut self) {
        let mut p = self.raw;
        unsafe {
            cpp!([ mut p as "QVariant*"] {
            p->clear();
        })
        }
    }

    /// convert value to specific type
    pub fn convert(&mut self, type_id: i32) -> bool {
        let mut p = self.raw;
        unsafe {
            cpp!([ mut p as "QVariant*", type_id as "int32_t"] -> bool as "bool" {
            return p->convert(type_id);
        })
        }
    }

    /// is this value empty?
    pub fn is_null(&self) -> bool {
        let p = self.raw;
        unsafe {
            cpp!([ p as "QVariant*"] -> bool as "bool" {
            return p->isNull();
        })
        }
    }

    /// is this value valid?
    pub fn is_valid(&self) -> bool {
        let p = self.raw;
        unsafe {
            cpp!([ p as "QVariant*"] -> bool as "bool" {
            return p->isValid();
        })
        }
    }

    // set the value of this QVariant
    pub fn set_value<T: Into<QVariant>>(&mut self, value :T) {
        let v: QVariant = value.into();
        let mut s = self.raw;

        unsafe {
            cpp! ([ v as "QVariant*",  mut s as "QVariant*"] {
            s->setValue(*v);
        })
        };

    }

    /// swaps the value with another QVariant
    pub fn swap(&mut self, value :&mut QVariant) {
        let mut v = value.raw;
        let mut s = self.raw;
        unsafe {
            cpp! ([ mut v as "QVariant*",  mut s as "QVariant*"] {
            s->swap(*v);
        })
        };
    }

}
impl Drop for QVariant {
    fn drop(&mut self) {
        let p = self.raw;
        unsafe {
            cpp! ([ p as "QVariant*"] {
            delete p;
        })
        };
    }
}
impl Default for QVariant {
    fn default() -> QVariant { QVariant::new() }
}
impl PartialEq for QVariant {
    fn eq(&self, other: &QVariant) -> bool {

        let p = self.raw;
        let o = other.raw;

        unsafe {
            cpp! ([ p as "QVariant*",  o as "QVariant*"] -> bool as "bool"{
            return (*p) == (*o);
        })
        }
    }
}


impl PartialOrd for QVariant {
    // TODO is this realy neccecary?
    fn partial_cmp(&self, other: &QVariant) -> Option<Ordering> {
        None
    }

    fn lt(&self, other: &QVariant) -> bool {
        let s = self.raw;
        let o = other.raw;
        unsafe {
            cpp! ([ s as "QVariant*", o as "QVariant*"]  -> bool as "bool"{
            return *s < *o;
        })}
    }
    fn le(&self, other: &QVariant) -> bool {
            let s = self.raw;
        let o = other.raw;
        unsafe {
            cpp! ([ s as "QVariant*", o as "QVariant*"]  -> bool as "bool"{
            return *s <= *o;
        })}
    }
    fn gt(&self, other: &QVariant) -> bool {
        let s = self.raw;
        let o = other.raw;
        unsafe {
            cpp! ([ s as "QVariant*", o as "QVariant*"]  -> bool as "bool"{
            return *s > *o;
        })}
    }
    fn ge(&self, other: &QVariant) -> bool {
        let s = self.raw;
        let o = other.raw;
        unsafe {
            cpp! ([ s as "QVariant*", o as "QVariant*"]  -> bool as "bool"{
            return *s >= *o;
        })}
    }
}


// -------------------------------------------------------
// TODO impl modelindex, stringlist, url, chrono::datetime, bytearray, map, qvariantlist, optinal; json

impl Into<bool> for QVariant {
    fn into(self) -> bool {
        let p = self.raw;
        unsafe {
            cpp!([ p as "QVariant*"] -> bool as "bool" {
            return p->toBool();
        })
        }
    }
}

impl Into<char> for QVariant {
    fn into(self) -> char {
        let p = self.raw;
        let u = unsafe {
            cpp!([ p as "QVariant*"] -> u16 as "uint16_t" {
            return p->toChar().unicode();
        })
        };
        let c =vec![u];
        decode_utf16(c.into_iter()).last().unwrap().unwrap()
    }
}


impl Into<i32> for QVariant {
    fn into(self) -> i32 {
        let p = self.raw;
        unsafe {
            cpp!([ p as "QVariant*"] -> i32 as "int" {
            return p->toInt();
        })
        }
    }
}
impl Into<f32> for QVariant {
    fn into(self) -> f32 {
        let p = self.raw;
        unsafe {
            cpp!([ p as "QVariant*"] -> f32 as "float" {
            return p->toFloat();
        })
        }
    }
}
impl Into<u32> for QVariant {
    fn into(self) -> u32 {
        let p = self.raw;
        unsafe {
            cpp!([ p as "QVariant*"] -> u32 as "uint" {
            return p->toUInt();
        })
        }
    }
}

impl Into<u64> for QVariant {
    fn into(self) -> u64 {
        let p = self.raw;
        unsafe {
            cpp!([ p as "QVariant*"] -> u64 as "qulonglong" {
            return p->toULongLong();
        })
        }
    }
}
impl Into<i64> for QVariant {
    fn into(self) -> i64 {
        let p = self.raw;
        unsafe {
            cpp!([ p as "QVariant*"] -> i64 as "long long" {
            return p->toLongLong();
        })
        }
    }
}
impl Into<f64> for QVariant {
    fn into(self) -> f64 {
        let p = self.raw;
        unsafe {
            cpp!([ p as "QVariant*"] -> f64 as "double" {
            return p->toDouble();
        })
        }
    }
}

impl Into<String> for QVariant {
    fn into(self) -> String {
        let p = self.raw;
        let mut s: usize = 0;

        let x = unsafe {
            cpp!([ p as "QVariant*", mut s as "size_t"] -> *mut c_char as "char*" {
                char* data = new char[p->toString().toUtf8().count()+1] ;
                strcpy(data, p->toString().toUtf8().constData());
                return data;
        })
        };
        let cs = unsafe { CString::from_raw(x)};
        cs.into_string().expect("Error at converting QVariant to String")
    }
}

// -------------------------------------------------------
// from
// TODO impl modelindex, stringlist, url, chrono::datetime, bytearray, map, qvariantlist, optinal; json
impl From<i32> for QVariant {
    fn from(v: i32) -> QVariant {
        let a = unsafe {
            cpp! ([v as "int32_t"] -> *mut c_void as "void *"{
            return new QVariant(v);
        })
        };
        QVariant { raw: a }
    }
}
impl From<u32> for QVariant {
    fn from(v: u32) -> QVariant {
        let a = unsafe {
            cpp! ([v as "uint32_t"] -> *mut c_void as "void *"{
            return new QVariant(v);
        })
        };
        QVariant { raw: a }
    }
}
impl From<i64> for QVariant {
    fn from(v: i64) -> QVariant {
        let a = unsafe {
            cpp! ([v as "qlonglong"] -> *mut c_void as "void *"{
            return new QVariant(v);
        })
        };
        QVariant { raw: a }
    }
}
impl From<u64> for QVariant {
    fn from(v: u64) -> QVariant {
        let a = unsafe {
            cpp! ([v as "qulonglong"] -> *mut c_void as "void *"{
            return new QVariant(v);
        })
        };
        QVariant { raw: a }
    }
}
impl From<f32> for QVariant {
    fn from(v: f32) -> QVariant {
        let a = unsafe {
            cpp! ([v as "float"] -> *mut c_void as "void *"{
            return new QVariant(v);
        })
        };
        QVariant { raw: a }
    }
}
impl From<f64> for QVariant {
    fn from(v: f64) -> QVariant {
        let a = unsafe {
            cpp! ([v as "double"] -> *mut c_void as "void *"{
            return new QVariant(v);
        })
        };
        QVariant { raw: a }
    }
}


impl From<String> for QVariant {
    fn from(v: String) -> QVariant {
        let d = CString::new(v).unwrap();
        let x = d.into_raw();

        let a = unsafe {
            cpp! ([x as "char*"] -> *mut c_void as "void *"{
            return new QVariant(x);
        })
        };
        unsafe {
            CString::from_raw(x);
        }
        QVariant { raw: a }

    }
}


#[test]
fn qvar_par_eq() {
    let a = QVariant::from(3);
    let b = QVariant::from("5".to_owned()  );

    assert!(a < b);
    assert!(a <= b);
    assert!(!(a > b));
    assert!(!(a >= b));
}

/*
#[test]
fn qvar_from() {
    let a = QVariant::from(-42i32);
    let a  = QVariant::from(42u32);

    let a  = QVariant::from(-42_000_000_000_000i64);
    let a  = QVariant::from(42_000_000_000_000u64);

}*/

#[test]
fn qvar_set_val() {
    let mut a = QVariant::new();
    a.set_value(42);
    let s:String = a.into();
    assert!(s == "42");
}

#[test]
fn qvar_swap() {
    let mut a = QVariant::from("a".to_owned());
    let mut b = QVariant::from(3);
    a.swap( &mut b);

    let x:i32 = a.into();
    let y:String = b.into();

    assert!(x == 3);
    assert!(y == "a");
}

