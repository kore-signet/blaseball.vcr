#!/bin/bash
./target/release/download_site_data tapes/site_data/ asset_replaces.toml
./target/release/build_entities -o tapes --whee idols risingstars
./target/release/build_entities -o tapes giftprogress
./target/release/build_entities -o tapes renovationprogress
./target/release/build_entities -o tapes globalevents offseasonsetup shopsetup offseasonrecap
./target/release/build_entities -o tapes vault
./target/release/build_entities -o tapes teamelectionstats decreeresult eventresult bonusresult
./target/release/build_entities -o tapes division subleague
./target/release/build_entities -o tapes player
./target/release/build_entities -o tapes item
./target/release/build_entities -o tapes --whee -d ./zstd-dictionaries/librarystory.dict librarystory
./target/release/build_entities -o tapes nullified
./target/release/build_entities -o tapes --whee fuelprogress
./target/release/build_entities -o tapes -c 100 team
./target/release/build_entities -o tapes --whee -c 100 -d zstd-dictionaries/sim.dict sim
./target/release/build_entities -o tapes -c 100 season standings temporal
./target/release/build_entities -o tapes -c 100 league subleague division tiebreakers
./target/release/build_entities -o tapes -c 100 tournament
./target/release/build_entities -o tapes -c 100 communitychestprogress
./target/release/build_entities -o tapes --whee -c 100 -d zstd-dictionaries/stadium.dict stadium
./target/release/build_entities -o tapes -c 100 playoffs playoffround playoffmatchup
./target/release/build_entities -o tapes -c 100 bossfight
./target/release/build_entities -o tapes -c 100 sunsun
./target/release/tributes