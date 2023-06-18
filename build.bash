#!/bin/bash
# ./target/release/download_site_data tapes/site_data/ asset_replaces.toml

# ./target/release/encode_vhs -c 100 -d vhs-dicts/player.dict -o vhs_tapes/player.vhs player
# ./target/release/encode_vhs -c 100 -d vhs-dicts/item.dict -o vhs_tapes/item.vhs item
# ./target/release/encode_vhs -c 100 -d vhs-dicts/renovationprogress.dict -o vhs_tapes/renovationprogress.vhs renovationprogress
# ./target/release/encode_vhs -c 100 -d vhs-dicts/bonusresult.dict -o vhs_tapes/bonusresult.vhs bonusresult
# ./target/release/encode_vhs -c 100 -d vhs-dicts/stadium.dict -o vhs_tapes/stadium.vhs stadium
# ./target/release/encode_vhs -c 100 -d vhs-dicts/team.dict -o vhs_tapes/team.vhs team

# ./target/release/encode_vhs -c 100 -o vhs_tapes/temporal.vhs temporal
# ./target/release/encode_vhs -c 100 -o vhs_tapes/standings.vhs standings
# ./target/release/encode_vhs -c 100 -o vhs_tapes/communitychestprogress.vhs communitychestprogress
# ./target/release/encode_vhs -c 100 -o vhs_tapes/fuelprogress.vhs fuelprogress
# ./target/release/encode_vhs -c 100 -o vhs_tapes/idols.vhs idols
# ./target/release/encode_vhs -c 100 -o vhs_tapes/sim.vhs sim
# ./target/release/encode_vhs -c 100 -o vhs_tapes/sunsun.vhs sunsun
# ./target/release/encode_vhs -c 100 -o vhs_tapes/bossfight.vhs bossfight
# ./target/release/encode_vhs -c 100 -o vhs_tapes/playoffs.vhs playoffs
# ./target/release/encode_vhs -c 100 -o vhs_tapes/playoffround.vhs playoffround
# ./target/release/encode_vhs -c 100 -o vhs_tapes/playoffmatchup.vhs playoffmatchup
# ./target/release/encode_vhs -c 100 -o vhs_tapes/season.vhs season
# ./target/release/encode_vhs -c 100 -o vhs_tapes/teamelectionstats.vhs teamelectionstats

# ./target/release/encode_vhs -o vhs_tapes/subleague.vhs subleague
# ./target/release/encode_vhs -o vhs_tapes/league.vhs league
# ./target/release/encode_vhs -o vhs_tapes/vault.vhs vault
# ./target/release/encode_vhs -o vhs_tapes/division.vhs division
# ./target/release/encode_vhs -o vhs_tapes/tournament.vhs tournament
# ./target/release/encode_vhs -o vhs_tapes/tiebreakers.vhs tiebreakers
# ./target/release/encode_vhs -o vhs_tapes/nullified.vhs nullified
# ./target/release/encode_vhs -o vhs_tapes/decreeresult.vhs decreeresult
# ./target/release/encode_vhs -o vhs_tapes/eventresult.vhs eventresult
# ./target/release/encode_vhs -o vhs_tapes/globalevents.vhs globalevents
# ./target/release/encode_vhs -o vhs_tapes/shopsetup.vhs shopsetup
# ./target/release/encode_vhs -o vhs_tapes/offseasonrecap.vhs offseasonrecap

./target/release/encode_vhs -o vhs_tapes/attributes.vhs attributes
./target/release/encode_vhs -o vhs_tapes/availablechampionbets.vhs availablechampionbets
./target/release/encode_vhs -o vhs_tapes/championcallout.vhs championcallout
./target/release/encode_vhs -o vhs_tapes/dayssincelastincineration.vhs dayssincelastincineration
./target/release/encode_vhs -o vhs_tapes/fanart.vhs fanart
./target/release/encode_vhs -o vhs_tapes/feedseasonlist.vhs feedseasonlist
./target/release/encode_vhs -o vhs_tapes/gamestatsheet.vhs gamestatsheet
./target/release/encode_vhs -o vhs_tapes/gammabracket.vhs gammabracket
./target/release/encode_vhs -o vhs_tapes/gammaelection.vhs gammaelection
./target/release/encode_vhs -o vhs_tapes/gammaelections.vhs gammaelections
./target/release/encode_vhs -o vhs_tapes/gammaelectiondetails.vhs gammaelectiondetails
./target/release/encode_vhs -o vhs_tapes/gammaelectionresults.vhs gammaelectionresults
./target/release/encode_vhs -o vhs_tapes/gammasim.vhs gammasim
./target/release/encode_vhs -o vhs_tapes/glossarywords.vhs glossarywords
./target/release/encode_vhs -o vhs_tapes/peanutpower.vhs peanutpower
./target/release/encode_vhs -o vhs_tapes/seasonstatsheet.vhs seasonstatsheet
./target/release/encode_vhs -o vhs_tapes/sponsordata.vhs sponsordata
./target/release/encode_vhs -o vhs_tapes/stadiumprefabs.vhs stadiumprefabs
./target/release/encode_vhs -o vhs_tapes/teamstatsheet.vhs teamstatsheet
./target/release/encode_vhs -o vhs_tapes/thebeat.vhs thebeat
./target/release/encode_vhs -o vhs_tapes/thebook.vhs thebook
./target/release/encode_vhs -c 500 -o vhs_tapes/playerstatsheet.vhs playerstatsheet


# ./target/release/build_entities -o tapes --whee idols risingstars
# ./target/release/build_entities -o tapes giftprogress
# ./target/release/build_entities -o tapes renovationprogress
# ./target/release/build_entities -o tapes globalevents offseasonsetup shopsetup offseasonrecap
# ./target/release/build_entities -o tapes vault
# ./target/release/build_entities -o tapes teamelectionstats decreeresult eventresult bonusresult
# ./target/release/build_entities -o tapes division subleague
# ./target/release/build_entities -o tapes player
# ./target/release/build_entities -o tapes item
# ./target/release/build_entities -o tapes --whee -d ./zstd-dictionaries/librarystory.dict librarystory
# ./target/release/build_entities -o tapes nullified
# ./target/release/build_entities -o tapes --whee fuelprogress
# ./target/release/build_entities -o tapes -c 100 team
# ./target/release/build_entities -o tapes --whee -c 100 -d zstd-dictionaries/sim.dict sim
# ./target/release/build_entities -o tapes -c 100 season standings temporal
# ./target/release/build_entities -o tapes -c 100 league subleague division tiebreakers
# ./target/release/build_entities -o tapes -c 100 tournament
# ./target/release/build_entities -o tapes -c 100 communitychestprogress
# ./target/release/build_entities -o tapes --whee -c 100 -d zstd-dictionaries/stadium.dict stadium
# ./target/release/build_entities -o tapes -c 100 playoffs playoffround playoffmatchup
# ./target/release/build_entities -o tapes -c 100 bossfight
# ./target/release/build_entities -o tapes -c 100 sunsun
# ./target/release/tributes