# this script computes the power level of each team in the json file
# the power level is calculated as the sum of the power level of the units in the synergy

if __name__ == "__main__":
    import json
    import sys
    import os
    import pandas
    """
    json file is an object where each key is a teamsize and the value is an array of teams with that size
    each team is a list of strings (the names of the units in the team)
    """
    # import the json file
    with open(sys.argv[1]) as f:
        psyns = json.load(f)
    # import the unit ranking data; the first row is the column names
    unit_ranking = pandas.read_csv(sys.argv[2], header=0)
    print(unit_ranking.head())
    # create a dictionary mapping unit name to composite ranking
    unit_ranking_dict = {}
    for index, row in unit_ranking.iterrows():
        print(row['champ_name'])
        unit_ranking_dict[row['champ_name']] = row['composite_score']
    # loop over the teams of each size, creating a new dictionary with the teams and their power levels
    teams_with_power_levels = {}
    for size in psyns:
        teams_with_power_levels[size] = []
        for team in psyns[size]:
            power_level = 0
            for unit in team:
                power_level += unit_ranking_dict[unit]
            teams_with_power_levels[size].append({"team":team, "power_level":power_level})
    # write the dictionary to a json file
    with open(os.path.split(sys.argv[1])[0] + '_power_levels.json', 'w') as f:
        json.dump(teams_with_power_levels, f)








