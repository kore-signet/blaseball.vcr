import struct
from genson import SchemaBuilder
import ujson
import pyzstd
from pprint import pprint

decompressor = pyzstd.EndlessZstdDecompressor()
builder = SchemaBuilder()

with open("../all_games.zstd", "rb") as gamesf:
    i = 0
    while compressed_len := gamesf.read(8):
        print(f"#{i}")
        compressed_len = struct.unpack("<Q", compressed_len)[0]
        uncompressed_len = struct.unpack("<Q", gamesf.read(8))[0]

        data = gamesf.read(compressed_len)
        data = decompressor.decompress(data, max_length=uncompressed_len)


        for item in ujson.loads(data):
            builder.add_object(item['data'])
    
        i += 1

with open("games.schema.json", "w") as outf:
    outf.write(builder.to_json())
        # break
        #     loop {
        # let mut len_buf: [u8; 8] = [0; 8];
        # if let Err(e) = reader.read_exact(&mut len_buf) {
        #     if e.kind() == io::ErrorKind::UnexpectedEof {
        #         break;
        #     } else {
        #         return Err(blaseball_vcr::VCRError::IOError(e));
        #     }
        # }

        # let compressed_len = u64::from_le_bytes(len_buf);
        # reader.read_exact(&mut len_buf)?;
        # let decompressed_len = u64::from_le_bytes(len_buf);

        # let mut buf: Vec<u8> = vec![0; compressed_len as usize];
        # reader.read_exact(&mut buf)?;
        # let decompressed = decompressor.decompress(&buf, decompressed_len as usize)?;

        # let deser_mrow = &mut serde_json::Deserializer::from_slice(&decompressed[..]);

        # // let game_d