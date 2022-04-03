#!/bin/bash
./target/release/train_vhs_dict -c 100 -o vhs-dicts/player.dict player
./target/release/train_vhs_dict -c 100 -o vhs-dicts/stadium.dict stadium
./target/release/train_vhs_dict -c 100 -o vhs-dicts/item.dict item
./target/release/train_vhs_dict -c 100 -o vhs-dicts/renovationprogress.dict renovationprogress
./target/release/train_vhs_dict -c 100 -o vhs-dicts/bonusresult.dict bonusresult
./target/release/train_vhs_dict -c 100 -o vhs-dicts/team.dict team
./target/release/train_vhs_dict -c 100 -o vhs-dicts/librarystory.dict librarystory
