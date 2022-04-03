use enum_map::EnumMap;
use fst::automaton::{StartsWith, Str as StrAutomaton};
use fst::{Automaton, IntoStreamer};

#[repr(u8)]
#[derive(enum_map::Enum)]
pub enum IndexType {
    PlayerTags,
    GameTags,
    TeamTags,
    EventType,
}

pub struct Index {
    indexes: EnumMap<IndexType, Option<fst::Map<Vec<u8>>>>,
}

impl Index {
    pub fn new() -> Index {
        Index {
            indexes: EnumMap::default(),
        }
    }

    pub fn add_index(&mut self, kind: IndexType, map: fst::Map<Vec<u8>>) {
        self.indexes[kind] = Some(map);
    }

    // lifetimes are my passion
    // explanation: the fst Streamer trait is a mess. we're bypassing it somewhat by returning a messy as hell concrete type
    pub fn get_by_tag_and_time<'f>(
        &'f self,
        idx_kind: IndexType,
        tag: &'f [u8],
        after: u32,
        before: u32,
    ) -> Option<fst::map::Stream<'_, StartsWith<StrAutomaton<'_>>>> {
        if let Some(idx) = &self.indexes[idx_kind] {
            let lower_bound = make_index_key(&tag, after);
            let upper_bound = make_index_key(&tag, before);

            Some(
                idx.search(
                    StrAutomaton::new(unsafe { std::str::from_utf8_unchecked(&tag) }).starts_with(),
                )
                .ge(lower_bound)
                .le(upper_bound)
                .into_stream(),
            )
        } else {
            None
        }
    }
}

/// convenience function to make an index key out of a tag + a time
#[inline(always)]
fn make_index_key(tag: &[u8], time: u32) -> Vec<u8> {
    [tag, &time.to_be_bytes()].concat()
}

/// takes a packed tuple of (u16 block index, u16 event index) and decomposes it into a proper tuple
#[inline(always)]
pub fn unpack_event_index(idx: u64) -> (u16, usize) {
    (
        (idx >> 16) as u16,                 // block index
        (idx & (u16::MAX as u64)) as usize, // event index
    )
}
