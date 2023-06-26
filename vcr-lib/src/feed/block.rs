use std::ops::Range;

use super::{event::*, BlockMetadata};

pub struct EventBlock {
    pub event_positions: Vec<(i64, u32)>,
    pub bytes: Vec<u8>,
    pub meta: BlockMetadata,
}

impl EventBlock {
    pub fn event_at_time(&self, at: i64) -> Option<EncodedFeedEvent<'_>> {
        let event_index = match self.event_positions.binary_search_by_key(&at, |(k, _)| *k) {
            Ok(i) => i,
            _ => return None,
        };

        self.event_at_index(event_index)
    }

    #[inline(always)]
    pub fn event_at_index(&self, event_index: usize) -> Option<EncodedFeedEvent<'_>> {
        let (timestamp, position) = match self.event_positions.get(event_index) {
            Some((timestamp, position)) => (*timestamp, *position),
            _ => return None,
        };

        // let archived = unsafe {
        //     rkyv::util::archived_value::<CompactedFeedEvent>(&self.bytes[..], position as usize)
        // };

        Some(EncodedFeedEvent::new(
            timestamp,
            &self.bytes[position as usize..],
            &self.meta,
        ))
    }

    pub fn events_at_time_range(
        &self,
        time_index: Range<i64>,
    ) -> Option<Vec<EncodedFeedEvent<'_>>> {
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
    pub fn all_events(&self) -> Vec<EncodedFeedEvent<'_>> {
        self.events_at_range(0..self.event_positions.len()).unwrap()
    }

    #[inline(always)]
    pub fn events_at_range(&self, event_index: Range<usize>) -> Option<Vec<EncodedFeedEvent<'_>>> {
        Some(
            self.event_positions
                .get(event_index)?
                .iter()
                .map(|(timestamp, position)| {
                    EncodedFeedEvent::new(*timestamp, &self.bytes[*position as usize..], &self.meta)
                })
                .collect(),
        )
    }
}
