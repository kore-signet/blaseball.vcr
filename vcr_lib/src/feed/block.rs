use super::event::*;

pub struct EventBlock {
    pub event_positions: Vec<(u64, u32)>,
    pub bytes: Vec<u8>,
}

impl EventBlock {
    pub fn event_at_time(&self, at: u64) -> Option<ArchivedEventWithTimestamp<'_>> {
        let event_index = match self.event_positions.binary_search_by_key(&at, |(k, _)| *k) {
            Ok(i) => i,
            _ => return None,
        };

        self.event_at_index(event_index)
    }

    #[inline(always)]
    pub fn event_at_index(&self, event_index: usize) -> Option<ArchivedEventWithTimestamp<'_>> {
        let (timestamp, position) = match self.event_positions.get(event_index) {
            Some((timestamp, position)) => (*timestamp, *position),
            _ => return None,
        };

        let archived = unsafe {
            rkyv::util::archived_value::<CompactedFeedEvent>(&self.bytes[..], position as usize)
        };

        Some(ArchivedEventWithTimestamp::new(timestamp, archived))
    }
}
