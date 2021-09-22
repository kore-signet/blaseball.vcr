use blaseball_vcr::feed::{CompactedFeedEvent, FeedEvent};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use uuid::Uuid;
//
fn main() {
    //     // let mut feed_dict: Vec<u8> = Vec::new();
    //     // let mut dict_f = File::open("zstd-dictionaries/feed.dict").unwrap();
    //     // dict_f.read_to_end(&mut feed_dict).unwrap();
    //     // let mut feed_compressor = zstd::block::Compressor::with_dict(feed_dict);
    let mut feed_samples: Vec<u8> = Vec::new();
    let mut feed_sample_lens: Vec<usize> = Vec::new();

    let mut player_tag_table: HashMap<Uuid, u16> = HashMap::new();
    let mut game_tag_table: HashMap<Uuid, u16> = HashMap::new();
    let mut team_tag_table: HashMap<Uuid, u8> = HashMap::new();

    let f = File::open("feed.json").unwrap();
    let reader = BufReader::new(f);

    for l in reader.lines() {
        let event: FeedEvent = serde_json::from_str(&l.unwrap()).unwrap();
        let compact_player_tags: Vec<u16> = event
            .player_tags
            .unwrap_or_default()
            .iter()
            .map(|id| {
                if let Some(n) = player_tag_table.get(id) {
                    *n
                } else {
                    let n = player_tag_table.len() as u16;
                    player_tag_table.insert(*id, n);
                    n
                }
            })
            .collect();

        let compact_game_tags: Vec<u16> = event
            .game_tags
            .unwrap_or_default()
            .iter()
            .map(|id| {
                if let Some(n) = game_tag_table.get(id) {
                    *n
                } else {
                    let n = game_tag_table.len() as u16;
                    game_tag_table.insert(*id, n);
                    n
                }
            })
            .collect();

        let compact_team_tags: Vec<u8> = event
            .team_tags
            .unwrap_or_default()
            .iter()
            .map(|id| {
                if let Some(n) = team_tag_table.get(id) {
                    *n
                } else {
                    let n = team_tag_table.len() as u8;
                    team_tag_table.insert(*id, n);
                    n
                }
            })
            .collect();

        let mut ev_bytes = CompactedFeedEvent {
            id: event.id,
            category: event.category,
            day: event.day,
            created: event.created,
            description: event.description,
            player_tags: compact_player_tags,
            game_tags: compact_game_tags,
            team_tags: compact_team_tags,
            etype: event.etype,
            tournament: event.tournament,
            metadata: event.metadata,
            phase: event.phase,
            season: event.season,
        }
        .encode();
        feed_sample_lens.push(ev_bytes.len());
        feed_samples.append(&mut ev_bytes);
    }

    //
    //     // let a: Vec<u8> = bincode::serialize(&trie).unwrap();
    // let a: Vec<u8> = trie.iter().map(|(k,v)| [k.clone(),v.to_be_bytes().to_vec()].concat()).flatten().collect();
    // println!("{:?}", zstd::encode_all(Cursor::new(a), 22).unwrap().len());
    //     //
    println!("making dict");
    let dict = zstd::dict::from_continuous(&feed_samples, &feed_sample_lens, 400_000).unwrap();
    let mut feed_dict_f = File::create("feed.dict").unwrap();
    feed_dict_f.write_all(&dict).unwrap();
}
