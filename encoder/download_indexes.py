from blaseball_mike import chronicler
import json
import ujson

index = {"by_pitcher": {}, "by_team": {}, "by_date": {}, "by_weather": {}}

ids_index = {"games": set(), "players": set(), "teams": set()}


def insert(key, id_, game):
    if not id_:
        return

    if id_ not in index[key]:
        index[key][id_] = []

    index[key][id_].append(game)


for game in chronicler.get_games():
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
    ids_index["games"].add(id_)

for player in chronicler.get_entities("player"):
    ids_index["players"].add(player["entityId"])

for team in chronicler.get_entities("team"):
    ids_index["teams"].add(team["entityId"])

i = 0

with open("../data/source/feed.json") as feed:
    for line in feed:
        if i % 10000 == 0:
            print(f"feed #{i}")

        event = ujson.loads(line) or {}
        for player in event.get("playerTags") or []:
            ids_index["players"].add(player)
        for team in event.get("teamTags") or []:
            ids_index["teams"].add(team)
        for game in event.get("gameTags") or []:
            ids_index["games"].add(game)
        i += 1


with open("games_index.json", "w") as f:
    f.write(json.dumps(index))

with open("ids_index.json", "w") as f:
    f.write(
        json.dumps(
            {
                "games": list(ids_index["games"]),
                "players": list(ids_index["players"]),
                "teams": list(ids_index["teams"]),
            }
        )
    )
