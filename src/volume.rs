
#[derive(Copy, Clone,Debug, Hash, PartialEq, Eq)]
pub struct CachedPartition{
    pub partno: u32,
    pub start_offset: u64,
    pub end_offset: u64,
}

pub struct Volume<S>{
   backing: S,
   part_cache: Vec<CachedPartition>,

}