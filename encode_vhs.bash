#!/bin/bash
./target/release/train_vhs_dict -o "./vhs-dicts/$1.dict" $1
./target/release/encode_vhs -o "./vhs_tapes/$1.vhs" -d "./vhs-dicts/$1.dict" $1