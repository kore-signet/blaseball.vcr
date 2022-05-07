from blaseball_mike import chronicler
import json

index = {
    "by_pitcher": {},
    "by_team": {},
    "by_date": {},
    "by_weather": {}
}

ids_index = {
    "games": [],
    "players": [],
    "teams": []
}

def insert(key, id_, game):
    if not id_:
        return

    if id_ not in index[key]:
        index[key][id_] = []
    
    index[key][id_].append(game)

for game in chronicler.get_games(before="2021-09-01T00:00:00Z"):
    id_ = game["gameId"]
    data = game["data"]
    pitchers = [data["awayPitcher"], data["homePitcher"]]
    teams = [data["awayTeam"], data["homeTeam"]]
    season = data["season"]
    day = data["day"]
    tournament = data.get("tournament", -1)
    weather = data["weather"]

    for pitcher in pitchers:
        insert("by_pitcher", pitcher, id_)
    
    for team in teams:
        insert("by_team", team, id_)
    
    insert("by_date", f"{day}:{season}:{tournament}", id_)
    insert("by_weather", weather, id_)
    ids_index["games"].append(id_)

for player in chronicler.get_entities("player", at="2021-09-01T00:00:00Z"):
    ids_index["players"].append(player["entityId"])

for team in chronicler.get_entities("team", at="2021-09-01T00:00:00Z"):
    ids_index["teams"].append(team["entityId"])

with open("games_index.json","w") as f:
    f.write(json.dumps(index))

with open("ids_index.json","w") as f:
    f.write(json.dumps(ids_index))