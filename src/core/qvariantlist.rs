use libc::c_void;

use core::QMetaTypeId;
use core::QVariant;
use std::ops::{Index,IndexMut};

cpp!{{
    #include <iostream>
    #include <QVariant>
    #include <QVariantList>
    #include <cstring>
}}

pub struct QVariantList{
    raw: *mut c_void,
}
impl QVariantList {
    // TODO
    pub fn new() {}

    // TODO
    pub fn insert(&mut self, index: usize, element: QVariant) {}

    // TODO
    pub fn remove(&mut self, index: usize) {}

    // TODO
    pub fn push(&mut self, value: QVariant) {}

    // TODO
    pub fn pop(&mut self) -> Option<QVariant> {None}

    // TODO
    pub fn append(&mut self,o: &mut QVariantList) {}

    // TODO
    pub fn clear() {}

    // TODO
    pub fn len() {}

    // TODO
    pub fn is_empty() {}

}

/*
// TODO
impl Index<usize> for QVariantList {
    type Output = QVariant;
    fn index(&self, index:usize) -> &QVariant{
        QVariant::new();
    }
}

// TODO
impl IndexMut<usize> for QVariantList {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut QVariant {
        QVariant::new()
    }
}
*/
