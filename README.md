# blaseball-vcr: miniaturizing blaseball
blaseball-vcr is a project to miniaturize blaseball history.

it works by downloading data from the [Society for Internet Blaseball Research's Chronicler API](https://github.com/xSke/Chronicler), storing it as a series of compressed 'patches', and then decompressing the data needed at runtime.

## how to use it
firstly, you need the data. you can either [download a pre-built one (extracts to about 600MB)](http://faculty.sibr.dev/~allie/tapes.tar.gz) or build your own dataset with:
```bash
cargo build --release

mkdir -p tapes/site_data
./build.bash
./target/release/build_games -d zstd-dictionaries/game_updates.dict -l <compression level> -t <number of threads to use> tapes
```
(note that this may take a while)

then, you can replay the data using the 'server' binary. it'll expose an API that mimicks Chronicler V2, making it compatible with tools like [before](https://github.com/iliana/before). make sure to set up a Vcr.toml file like the one in this repository!
