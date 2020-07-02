// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod meta {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

pub enum MetaOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Meta<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Meta<'a> {
    type Inner = Meta<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Meta<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Meta {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args MetaArgs<'args>) -> flatbuffers::WIPOffset<Meta<'bldr>> {
      let mut builder = MetaBuilder::new(_fbb);
      if let Some(x) = args.refs { builder.add_refs(x); }
      builder.finish()
    }

    pub const VT_REFS: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn refs(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<&'a str>>>>(Meta::VT_REFS, None)
  }
}

pub struct MetaArgs<'a> {
    pub refs: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<&'a  str>>>>,
}
impl<'a> Default for MetaArgs<'a> {
    #[inline]
    fn default() -> Self {
        MetaArgs {
            refs: None,
        }
    }
}
pub struct MetaBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MetaBuilder<'a, 'b> {
  #[inline]
  pub fn add_refs(&mut self, refs: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Meta::VT_REFS, refs);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MetaBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MetaBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Meta<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

#[inline]
pub fn get_root_as_meta<'a>(buf: &'a [u8]) -> Meta<'a> {
  flatbuffers::get_root::<Meta<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_meta<'a>(buf: &'a [u8]) -> Meta<'a> {
  flatbuffers::get_size_prefixed_root::<Meta<'a>>(buf)
}

#[inline]
pub fn finish_meta_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Meta<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_meta_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Meta<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod meta

