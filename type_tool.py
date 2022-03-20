import sys
import subprocess
from blaseball_mike import chronicler
from genson import SchemaBuilder
import io

schema = SchemaBuilder()
ty = sys.argv[1] # entity type

i = 0
for version in chronicler.get_versions(ty, before="2021-09-01T00:00:00Z"):
    schema.add_object(version['data'])
    i += 1
    print(f"#{ty} - #{i}")

subprocess.run(["quicktype", "-s", "schema", "-", "-l", "rust", "--visibility", "public", "-o", f"./vcr_lib/src/vhs/schemas/{ty}.rs"], input=schema.to_json(), encoding="utf8")