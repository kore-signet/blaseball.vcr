use crate::db_manager::*;
use crate::feed::lookup_tables::GAME_TO_UUID;
use crate::game_lookup_tables::*;
use crate::vhs::schemas::{self, *};
use crate::*;
use compiled_uuid::uuid;
use uuid::Uuid;

macro_rules! fetch_batch {
    (all of $what:ty; from $db:expr; at $at:expr) => {
        fetch_batch!($what; from $db; with $db.all_entity_ids::<$what>().ok_or(VCRError::EntityTypeNotFound)?; at $at)
    };
    (all of $what:ty; from $db:expr; at $at:expr; clamped) => {
        fetch_batch!($what; from $db; with $db.all_entity_ids::<$what>().ok_or(VCRError::EntityTypeNotFound)?; at $at; clamped)
    };
    ($what:ty; from $db:expr; with $ids:expr; at $at:expr; clamped) => {
        fetch_batch!($what; from $db; with $ids; at clamp($at, 1599169238, u32::MAX))
    };
    ($what:ty; from $db:expr; with $ids:expr; at $at:expr) => {
        $db
            .get_entities::<$what>(
                $ids,
                $at,
            )?
            .into_iter()
            .filter_map(|v| v.map(|inner| inner.data))
            .collect()
    }
}

fn clamp<T: std::cmp::Ord>(val: T, min: T, max: T) -> T {
    if val > max {
        max
    } else if val < min {
        min
    } else {
        val
    }
}

fn games_by_date_and_time(
    db: &impl EntityDatabase<Record = GameUpdate>,
    date: &GameDate,
    at: u32,
) -> VCRResult<Vec<OptionalEntity<GameUpdate>>> {
    if let Some(ids) = DATES_TO_GAMES.get(&date.to_bytes()) {
        let game_ids: Vec<[u8; 16]> = ids
            .into_iter()
            .map(|tag| *(GAME_TO_UUID[*tag as usize]).as_bytes())
            .collect();
        db.get_entities(&game_ids, at)
    } else {
        Ok(vec![])
    }
}

fn games_for_bets(
    db: &impl EntityDatabase<Record = GameUpdate>,
    date: &GameDate,
    at: u32,
) -> VCRResult<Vec<OptionalEntity<GameUpdate>>> {
    if let Some(ids) = DATES_TO_GAMES.get(&date.to_bytes()) {
        let game_ids: Vec<[u8; 16]> = ids
            .into_iter()
            .map(|tag| *(GAME_TO_UUID[*tag as usize]).as_bytes())
            .collect();

        let mut res = Vec::with_capacity(game_ids.len());

        'outer: for id in game_ids {
            let mut game = match db.get_entity(&id, at)?.or(db.get_first_entity(&id)?) {
                Some(v) => v,
                None => continue 'outer,
            };

            let mut time = game.valid_from;

            while game.data.away_odds == 0f64 && game.data.home_odds == 0f64 {
                time = db.get_next_time(&id, time).unwrap();
                game = db.get_entity(&id, time)?.unwrap();
            }

            res.push(Some(game));
        }

        Ok(res)
    } else {
        Ok(vec![])
    }
}

fn playoffs(
    db: &DatabaseManager,
    playoff_id: &[u8; 16],
    round: Option<i64>,
    at: u32,
) -> VCRResult<PlayoffData> {
    let playoffs = db
        .get_entity::<Playoffs>(playoff_id, at)?
        .ok_or(VCRError::EntityNotFound)?
        .data;
    let round_number: i64 = round.or(playoffs.round).unwrap();
    let round_ids: Vec<[u8; 16]> = playoffs.rounds.iter().map(|v| *v.as_bytes()).collect();
    let rounds: Vec<Playoffround> = db
        .get_entities::<Playoffround>(&round_ids, at)?
        .into_iter()
        .filter_map(|v| v.map(|inner| inner.data))
        .collect();
    let tomorrow_round = if let Some(tomorrow_round_n) = playoffs.tomorrow_round {
        rounds
            .iter()
            .find(|v| v.round_number == tomorrow_round_n)
            .cloned()
    } else {
        None
    };

    let current_round = rounds
        .iter()
        .find(|v| v.round_number == round_number)
        .cloned();

    let main_matchup_ids: Vec<[u8; 16]> = current_round
        .as_ref()
        .map(|v| v.matchups.iter().map(|v| *v.as_bytes()).collect())
        .unwrap_or_default();
    let main_matchups = db
        .get_entities::<Playoffmatchup>(&main_matchup_ids, at)?
        .into_iter()
        .filter_map(|v| v.map(|inner| inner.data))
        .collect();

    let tomorrow_matchup_ids: Vec<[u8; 16]> = tomorrow_round
        .as_ref()
        .map(|v| v.matchups.iter().map(|v| *v.as_bytes()).collect())
        .unwrap_or_default();
    let tomorrow_matchups = db
        .get_entities::<Playoffmatchup>(&tomorrow_matchup_ids, at)?
        .into_iter()
        .filter_map(|v| v.map(|inner| inner.data))
        .collect();

    let all_matchup_ids: Vec<[u8; 16]> = rounds
        .iter()
        .flat_map(|r| r.matchups.iter().map(|id| *id.as_bytes()))
        .collect();
    let all_matchups = db
        .get_entities::<Playoffmatchup>(&all_matchup_ids, at)?
        .into_iter()
        .filter_map(|v| v.map(|inner| inner.data))
        .collect();

    Ok(PlayoffData {
        round: current_round,
        matchups: main_matchups,
        playoffs,
        all_rounds: rounds,
        all_matchups,
        tomorrow_round,
        tomorrow_matchups,
    })
}

pub fn stream_data(db: &DatabaseManager, at: u32) -> VCRResult<StreamDataWrapper> {
    let sim = db
        .get_entity::<Sim>(Uuid::nil().as_bytes(), at)?
        .ok_or(VCRError::EntityNotFound)?
        .data;

    let mut working_date = GameDate {
        season: sim.season as i8,
        day: sim.day as i16,
        tournament: if sim.season == 10 && sim.day < 100 && sim.tournament.is_none() {
            -1
        } else {
            sim.tournament.unwrap_or(-1) as i8
        },
    };

    if working_date.tournament != -1 {
        working_date.season = -1;
    }

    let game_db = db
        .get_db::<GameUpdate>()
        .ok_or(VCRError::EntityTypeNotFound)?
        .as_any()
        .downcast_ref::<crate::vhs::db::Database<GameUpdate>>()
        .unwrap();

    let schedule: Vec<GameUpdate> = if sim.phase == 14 && working_date.season == 22 {
        if let Some(data) = db
            .get_entity::<GameUpdate>(
                &uuid!("d162b23a-9832-4e78-8d78-5d131393fd61").as_bytes(),
                at,
            )?
            .map(|v| v.data)
        {
            vec![data]
        } else {
            vec![]
        }
    } else {
        games_by_date_and_time(game_db, &working_date, at)?
            .into_iter()
            .filter_map(|v| v.map(|a| a.data))
            .collect()
    };

    working_date.day += 1;

    let tomorrow_schedule: Vec<GameUpdate> = games_for_bets(game_db, &working_date, at)?
        .into_iter()
        .filter_map(|v| v.map(|a| a.data))
        .collect();

    let season = db
        .get_entities::<Season>(
            db.all_entity_ids::<Season>()
                .ok_or(VCRError::EntityTypeNotFound)?,
            at,
        )?
        .into_iter()
        .find(|season| {
            if let Some(s) = season {
                s.data.season_number == sim.season
            } else {
                false
            }
        })
        .ok_or(VCRError::EntityNotFound)?
        .unwrap()
        .data;

    let standings = db
        .get_entity::<Standings>(season.standings.as_bytes(), at)?
        .map(|v| v.data);

    let leagues: Vec<League> = fetch_batch!(all of League; from db; at at; clamped);
    let subleague_ids: Vec<[u8; 16]> = leagues
        .iter()
        .flat_map(|league| league.subleagues.iter().map(|id| *id.as_bytes()))
        .collect();
    let tiebreaker_ids: Vec<[u8; 16]> = leagues
        .iter()
        .map(|league| *league.tiebreakers.as_bytes())
        .collect();

    let tiebreakers: Vec<TiebreakerWrapper> =
        fetch_batch!(TiebreakerWrapper; from db; with &tiebreaker_ids; at at; clamped);
    let subleagues: Vec<Subleague> =
        fetch_batch!(Subleague; from db; with &subleague_ids; at at; clamped);

    let division_ids: Vec<[u8; 16]> = subleagues
        .iter()
        .flat_map(|league| league.divisions.iter().map(|id| *id.as_bytes()))
        .collect();

    let divisions: Vec<Division> =
        fetch_batch!(Division; from db; with &division_ids; at at; clamped);

    let teams: Vec<Team> = fetch_batch!(all of Team; from db; at at);

    let stadiums: Vec<Stadium> = fetch_batch!(all of Stadium; from db; at at);

    let mut boss_fights: Vec<Bossfight> = fetch_batch!(all of Bossfight; from db; at at);
    boss_fights.retain(|fight| fight.home_hp != "0" && fight.away_hp != "0");

    let temporal = db
        .get_entity::<Temporal>(Uuid::nil().as_bytes(), at)?
        .map(|v| v.data);
    let sunsun = db
        .get_entity::<Sunsun>(Uuid::nil().as_bytes(), at)?
        .map(|v| v.data);
    let community_chest = db
        .get_entity::<CommunityChestProgress>(Uuid::nil().as_bytes(), at)?
        .map(|v| v.data);

    let tournament = if working_date.tournament > -1 {
        db.get_entities::<Tournament>(
            &db.all_entity_ids::<Tournament>()
                .ok_or(VCRError::EntityTypeNotFound)?,
            at,
        )?
        .pop()
        .flatten()
        .map(|v| v.data)
    } else {
        None
    };

    let mut postseason: Option<PlayoffData> = None;
    let mut postseasons: Option<Vec<PlayoffData>> = None;

    if let Some(ref tourn) = tournament {
        postseason = Some(playoffs(
            &db,
            tourn.playoffs.as_bytes(),
            sim.tournament_round,
            at,
        )?);
    } else {
        match sim.playoffs {
            schemas::sim::Playoffs::String(ref id) => {
                postseason = Some(playoffs(&db, id.as_bytes(), sim.play_off_round, at)?);
            }
            schemas::sim::Playoffs::StringArray(ref ids) => {
                let mut res = Vec::with_capacity(ids.len());

                for id in ids {
                    res.push(playoffs(&db, id.as_bytes(), sim.play_off_round, at)?);
                }

                postseasons.replace(res);
            }
        };
    };

    Ok(StreamDataWrapper {
        value: StreamData {
            games: GameData {
                sim,
                season,
                schedule,
                tomorrow_schedule,
                tournament,
                standings,
                postseason,
                postseasons,
            },
            leagues: LeagueData {
                teams,
                subleagues,
                divisions,
                leagues,
                tiebreakers,
                stadiums,
                stats: StatData {
                    sunsun,
                    community_chest,
                },
            },
            fights: FightData { boss_fights },
            temporal,
        },
    })
}
