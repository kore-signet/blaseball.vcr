#!/bin/bash
./target/release/download_site_data tapes/site_data/
./target/release/build_entities -1 nodict idols risingstars
./target/release/build_entities -1 nodict tributes
./target/release/build_entities -1 nodict communitychestprogress
./target/release/build_entities -1 nodict giftprogress
./target/release/build_entities -1 nodict renovationprogress
./target/release/build_entities -1 nodict globalevents offseasonsetup shopsetup offseasonrecap
./target/release/build_entities -1 nodict sunsun vault
./target/release/build_entities -1 nodict teamelectionstats decreeresult eventresult bonusresult
./target/release/build_entities -1 nodict division subleague
./target/release/build_entities -1 nodict player
./target/release/build_entities -1 nodict item
./target/release/build_entities 100 nodict team
./target/release/build_entities 100 nodict sim season standings temporal
./target/release/build_entities 100 nodict league subleague division tiebreakers
./target/release/build_entities 100 nodict tournament
./target/release/build_entities 100 nodict communitychestprogress
./target/release/build_entities 100 nodict stadium
./target/release/build_entities 100 nodict playoffs playoffround playoffmatchup
./target/release/build_entities 100 nodict bossfight
