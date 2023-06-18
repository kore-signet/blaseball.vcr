import requests
import json

updates = requests.get("https://api.sibr.dev/chronicler/v1/site/updates").json()["data"]
files = {
    'maincss': [],
    'mainjs': [],
    'indexhtml': [],
    '2js': []
}

for i,update in enumerate(updates):
    file = update["downloadUrl"].split("/")[-1]
    ext = file.split(".")[-1]
    name = file.split(".")[0]
    print(f"#{i} - {name}{ext}/{update['timestamp']}.{ext}")

    r = requests.get(f"https://api.sibr.dev/chronicler/v1{update['downloadUrl']}")
    r.raise_for_status()
    with open(f"{name}{ext}/{update['timestamp']}.{ext}", "wb") as fd:
        for chunk in r.iter_content(chunk_size=128):
            fd.write(chunk)
    
    update["where"] = f"{name}{ext}/{update['timestamp']}.{ext}"
    files[f"{name}{ext}"].append(update)

with open("files.json", "w") as outf:
    outf.write(json.dumps(files))
