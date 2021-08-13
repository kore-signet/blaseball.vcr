#!/bin/bash
./target/release/download_site_data tapes/site_data/ asset_replaces.toml
./target/release/build_entities -1 nodict idols risingstars
./target/release/build_entities -1 ./zstd-dictionaries/tributes.dict tributes
./target/release/build_entities -1 nodict giftprogress
./target/release/build_entities -1 nodict renovationprogress
./target/release/build_entities -1 nodict globalevents offseasonsetup shopsetup offseasonrecap
./target/release/build_entities -1 nodict sunsun vault
./target/release/build_entities -1 nodict teamelectionstats decreeresult eventresult bonusresult
./target/release/build_entities -1 nodict division subleague
./target/release/build_entities -1 nodict player
./target/release/build_entities -1 nodict item
./target/release/build_entities -1 ./zstd-dictionaries/librarystory.dict librarystory
./target/release/build_entities -1 nodict nullified
./target/release/build_entities -1 nodict fuelprogress
./target/release/build_entities 100 nodict team
./target/release/build_entities 100 zstd-dictionaries/sim.dict sim
./target/release/build_entities 100 season standings temporal
./target/release/build_entities 100 nodict league subleague division tiebreakers
./target/release/build_entities 100 nodict tournament
./target/release/build_entities 100 nodict communitychestprogress
./target/release/build_entities 100 zstd-dictionaries/stadium.dict stadium
./target/release/build_entities 100 nodict playoffs playoffround playoffmatchup
./target/release/build_entities 100 nodict bossfight
