import json
import multiprocessing
from pathlib import Path
from multiprocessing import Pool
import itertools
from re import M
import heapq
import functools
import re
import numpy as np
import queue 
import random
from functools import partial

def load_breakpoints(path):
    """
    path: Path object pointing to a file with breakpoints
    each line should be of the form
    trait, breakpoint1, breakpoint2, ..., breakpointN
    for example Arcanist, 2, 4, 6, 8
    returns a dict mapping trait name to a set of breakpoints
    an example dict entry would be
    "Arcanist": {0, 2, 4, 6, 8}
    zero is appended even through it wasn't in the input file because of the condition for a perfect synergy:
    a team is perfect if:
    for every trait in the game
    the frequency of that trait in the team is in the breakpoints set for the trait
    """
    with path.open() as f:
        breakpoints = f.readlines()
    breakpoints = [[x.strip() for x in breakpoint.split(",")] for breakpoint in breakpoints]
    traits = [x[0] for x in breakpoints]
    breakpoints = [[int(y) for y in x[1:]] for x in breakpoints]
    breakpoints = {trait:breakpoint for trait, breakpoint in zip(traits, breakpoints)}
    for trait in breakpoints:
        breakpoints[trait].append(0)
        breakpoints[trait] = set(breakpoints[trait])
    return breakpoints
def load_units(path):
    """
    Loads units from a file.
    path: Path object
    get back
    units: dict int -> list of int showing which traits the unit has
    unit_names: dict int -> string mapping unit id to name
    unit_names_inv: dict string -> int mapping unit name to id
    traits: dict int -> string mapping trait id to name
    traits_inv: dict string -> int mapping trait name to id
    """
    with open(path, "r") as f:
        units = f.readlines()
    # parse each line
    data = [unit.split(",") for unit in units]
    units = {}
    unit_names = {}
    unit_names_inv = {}
    traits = {}
    traits_inv = {}
    # map id to unit name
    for i, unit in enumerate(data):
        units[i] = list(map(lambda x: x.strip(), unit[2:6]))
        unit_names[i] = unit[0]
        unit_names_inv[unit[0]] = i
    # collect all unique traits
    all_traits = set()
    for unit in units:
        all_traits.update(units[unit])
    # map id to trait name
    for i, trait in enumerate(all_traits):
        traits[i] = trait
        traits_inv[trait] = i
    # convert unit traits to trait ids
    for unit in units:
        units[unit] = [traits_inv[trait] for trait in units[unit]]
    return units, unit_names, unit_names_inv, traits, traits_inv

def is_perfect_synergy(team, units, trait_breaks, team_dict):
    for unit in team:
        # add units traits to team_dict
        for trait in units[unit]:
            team_dict[trait] += 1
    perfect = 1
    for i in range(len(trait_breaks)):
        # for each trait, check that the team has a <BREAKPOINT> number of units with that trait
        if perfect and team_dict[i] not in trait_breaks[i]:
            perfect = 0
        team_dict[i] = 0 
    return perfect

def set_6_5_perfect_synergy():
    """
    returns a function that does the following
    takes a team (list of ints representing unit ids) and returns 1 if it is a perfect synergy
    """
    units_data_path = Path("champs.csv")
    units, _, _, _, traits_inv = load_units(units_data_path)
    # load breakpoints
    breakpoints_path = Path("traits.csv")
    breakpoints_sk = load_breakpoints(breakpoints_path)
    trait_breaks = {traits_inv[k]: v for k, v in breakpoints_sk.items()}
    team_dict = dict.fromkeys(trait_breaks.keys(), 0)
    return functools.partial(is_perfect_synergy, units=units, trait_breaks=trait_breaks, team_dict=team_dict)

def best_of_size(units, size, measure, top_n, workers = 12, chunksize = 10000):
    """
    returns the top n teams of size size using measure to evaluate the teams
    """
    p = Pool(processes=workers)
    combs = itertools.combinations(units.keys(), size)
    # copy of combs iterator
    first_combs, second_combs = itertools.tee(combs)
    # compute the teams in parallel
    all_teams_it = zip(p.imap(measure, first_combs, chunksize=chunksize), second_combs)
    #all_teams_it = zip(map(measure, first_combs), second_combs)
    r = heapq.nlargest(top_n, all_teams_it, key=lambda x: x[0])
    p.close()
    return r
def all_perfect_synergies(minsize, maxsize, top_n):
    """
    returns a dictionary
    each key is a size of team
    each value is a list of top_n teams of that size
    """
    def team_id_to_name(team, unit_names):
        return [unit_names[unit] for unit in team]
    def teams_to_names(teams, unit_names):
        return [(team_id_to_name(team[1], unit_names), team[0]) for team in teams]
    units_data_path = Path("champs.csv")
    units, unit_dict, _, _, traits_inv = load_units(units_data_path)
    perfect = set_6_5_perfect_synergy()
    team_comps = {}
    for team_size in range(minsize, maxsize + 1):
        team_comps[team_size] = teams_to_names(best_of_size(units, team_size, perfect, top_n), unit_dict)
        print(f"Finished with team of size {team_size}")
    return team_comps

def main():
    r = all_perfect_synergies(4, 9, 500)
    # save to file, creating if necessary
    with open("perfect_synergies.json", "w") as f:
        json.dump(r, f)
    for i in r:
        print("Comps of size", i)
        print(r[i][:20])
        print("\n")
        print("\n")

    
if __name__ == "__main__":
    main()

