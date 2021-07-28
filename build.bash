#!/bin/bash
# ./target/release/download_site_data tapes/site_data/
./target/release/build_entities -1 idols risingstars
./target/release/build_entities -1 tributes
./target/release/build_entities -1 communitychestprogress
./target/release/build_entities -1 giftprogress
./target/release/build_entities -1 renovationprogress
./target/release/build_entities -1 globalevents offseasonsetup shopsetup offseasonrecap
./target/release/build_entities -1 sunsun vault
./target/release/build_entities -1 teamelectionstats decreeresult eventresult bonusresult
./target/release/build_entities -1 division subleague
./target/release/build_entities -1 player
./target/release/build_entities -1 item
./target/release/build_entities 100 team
./target/release/build_entities 100 sim season standings temporal
./target/release/build_entities 100 league subleague division tiebreakers
./target/release/build_entities 100 tournament
./target/release/build_entities 100 communitychestprogress
./target/release/build_entities 100 stadium
./target/release/build_entities 100 playoffs playoffround playoffmatchup
./target/release/build_entities 100 bossfight
