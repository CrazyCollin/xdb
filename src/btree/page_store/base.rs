use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) const MAX_VALUE_LENGTH:usize=3*1024*1024*1024;
pub(crate) const MAX_PAGE_INDEX:u32=0x000F_FFFF;

#[derive(Ord, PartialOrd, Eq, PartialEq,Hash,Copy, Clone)]
pub(crate) struct PageNumber{
    pub(crate) region:u32,
    pub(crate) page_index:u32,
    pub(crate) page_order:u8,
}

impl PageNumber {
    #[inline(always)]
    pub(crate) const fn serialized_size()->usize{8}

    pub(crate) fn new(region:u32,page_index:u32,page_order:u8)->Self{
        debug_assert!(region<=0x000F_FFFF);
        debug_assert!(page_index<=MAX_PAGE_INDEX);
        debug_assert!(page_order<=20);
        Self{
            region,
            page_index,
            page_order,
        }
    }


}


pub(crate) trait Page{
    fn memory(&self)->&[u8];

    fn get_page_number(&self)->PageNumber;
}

pub struct PageImpl<'a>{
    pub(super) mem:Arc<Vec<u8>>,
    pub(super) page_number:PageNumber,
    pub(super) open_pages:&'a Mutex<HashMap<PageNumber,u64>>,
}

