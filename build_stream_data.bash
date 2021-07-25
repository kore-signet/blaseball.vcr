#!/bin/bash
# todo: game updates
./target/release/build_entities tapes/stream_data/ sim season standings temporal
./target/release/build_entities tapes/stream_data/ league subleague division tiebreakers
./target/release/build_entities tapes/stream_data/ tournament
./target/release/build_entities tapes/stream_data/ communitychestprogress
./target/release/build_entities tapes/stream_data/ stadium
./target/release/build_entities tapes/stream_data/ playoffs playoffround playoffmatchup
./target/release/build_entities tapes/stream_data/ bossfight
