use std::ops::Range;

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

    pub fn events_at_time_range(
        &self,
        time_index: Range<u64>,
    ) -> Option<Vec<ArchivedEventWithTimestamp<'_>>> {
        let start = match self
            .event_positions
            .binary_search_by_key(&time_index.start, |(k, _)| *k)
        {
            Ok(i) => i,
            Err(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
        };

        let end = match self
            .event_positions
            .binary_search_by_key(&time_index.end, |(k, _)| *k)
        {
            Ok(i) => i,
            Err(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
        };

        self.events_at_range(start..end)
    }

    #[inline(always)]
    pub fn all_events(&self) -> Vec<ArchivedEventWithTimestamp<'_>> {
        self.events_at_range(0..self.event_positions.len()).unwrap()
    }

    #[inline(always)]
    pub fn events_at_range(
        &self,
        event_index: Range<usize>,
    ) -> Option<Vec<ArchivedEventWithTimestamp<'_>>> {
        Some(
            self.event_positions
                .get(event_index)?
                .iter()
                .map(|(timestamp, position)| {
                    ArchivedEventWithTimestamp::new(*timestamp, unsafe {
                        rkyv::util::archived_value::<CompactedFeedEvent>(
                            &self.bytes[..],
                            *position as usize,
                        )
                    })
                })
                .collect(),
        )
    }
}
