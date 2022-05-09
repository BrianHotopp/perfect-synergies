// hashmap and hashset
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
// Combinations
use itertools::Itertools;


use rayon::prelude::*;
// _file reading and writing
use std::fs;
use std::fs::File;
use std::io::Write;


fn read_champs(_file: &str) -> (HashMap<u8, String>, HashMap<String, u8>) {
    // reads the champion names from the _file
    // returns:
    // 1. a hashmap mapping integers to the names of the champions
    // 2. a hashmap mapping names of the champions to their integer
    let mut champs = HashMap::new();
    let mut champs_rev = HashMap::new();
    let contents = fs::read_to_string(_file).unwrap();
    let mut lines = contents.lines();
    let mut i = 0;
    while let Some(line) = lines.next() {
        // each line has the form
        // <champion name>, <cost>, <trait 1>, <trait 2> ... <trait n>
        // we just want to extract the name
        let mut line = line.split(",");
        let name = line.next().unwrap().trim();
        champs.insert(i, name.to_string());
        champs_rev.insert(name.to_string(), i);
        i += 1;
    }
    (champs, champs_rev)
}
fn read_traits(_file: &str) -> (HashMap<u8, String>, HashMap<String, u8>) {
    // reads the traits from the _file
    // returns:
    // 1. a hashmap mapping integers to the names of the traits
    // 2. a hashmap mapping names of the traits to their integer
    let mut traits = HashMap::new();
    let mut traits_rev = HashMap::new();
    let contents = fs::read_to_string(_file).unwrap();
    let mut lines = contents.lines();
    let mut i = 0;
    while let Some(line) = lines.next() {
        // each line has the form
        // <trait name>, <break1>, <break2> ... <break n>
        // we just want to extract the name
        let mut line = line.split(",");
        let name = line.next().unwrap().trim();
        traits.insert(i, name.to_string());
        traits_rev.insert(name.to_string(), i);
        i += 1;
    }
    (traits, traits_rev)
}
fn read_breaks(_file: &str, traits: &HashMap<String, u8>) -> HashMap<u8, HashSet<u8>> {
    // reads the breaks from the _file
    // returns:
    // 1. a hashmap mapping traits (by id) to their breakpoints
    let mut breaks = HashMap::new();
    let contents = fs::read_to_string(_file).unwrap();
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        // each line has the form
        // <trait name>, <break1>, <break2> ... <break n>
        // we just want to extract the name and the breaks
        let mut line = line.split(",");
        let name = line.next().unwrap();
        let trait_id = traits.get(name).unwrap();
        let mut breaks_set = HashSet::new();
        while let Some(break_str) = line.next() {
            // convert the breakpoint to an integer and add it to the set
            breaks_set.insert(break_str.trim().parse::<u8>().unwrap());
        }
        breaks.insert(*trait_id, breaks_set);
    }
    // return the hashmap
    breaks
}
fn read_champ_traits(_file: &str, champs_rev: &HashMap<String, u8>, traits_rev: &HashMap<String, u8>) -> HashMap<u8, Vec<u8>> {
    // reads the champ_traits from the _file
    // returns:
    // 1. a hashmap mapping champions (by id) to their traits
    let mut champ_traits = HashMap::new();
    let contents = fs::read_to_string(_file).unwrap();
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        // each line has the form
        // <champion name>, <cost>, <trait 1>, <trait 2> ... <trait n>
        // we just want to extract the name and the traits
        let mut line = line.split(",");
        let name = line.next().unwrap();
        let champ_id = champs_rev.get(name).unwrap();
        let mut traits_vec = Vec::new();
        // skip the cost
        line.next();
        while let Some(trait_str) = line.next() {
            // convert the trait to an integer and add it to the vector
            traits_vec.push(*traits_rev.get(trait_str.trim()).unwrap());
        }
        champ_traits.insert(*champ_id, traits_vec); 
    }
    // return the hashmap
    champ_traits
}

fn is_perfect_synergy(team: &Vec<&u8>, traits: &HashMap<u8, String>, unit_traits: &HashMap<u8, Vec<u8>>,breaks: &HashMap<u8, HashSet<u8>>) -> bool {
    // checks if the team is a perfect synergy
    // returns:
    // true if the team is a perfect synergy
    // false otherwise
    // build up a dict of the traits the team has
    // initialize with keys from the traits dict values
    let mut team_traits = HashMap::new();
    for trait_id in traits.keys() {
        team_traits.insert(*trait_id, 0);
    }
    for nit in team {
        // for each unit in the team get the traits
        let unit_traits = &unit_traits[*nit];
        // increment the count for each trait
        for trait_id in unit_traits {
            team_traits.insert(*trait_id, team_traits[trait_id] + 1);
        }
    }
    // for each trait in the team
    // if the count is not in the breaks for the trait
    // return false
    for (trait_id, count) in team_traits.iter_mut() {
        if !breaks.get(trait_id).unwrap().contains(count) {
            return false;
        }
    }
    // if we get here, the team is a perfect synergy
    true
}
fn do_perfect_synergies(champs: &HashMap<u8, String>, traits: &HashMap<u8, String>, champtraits: &HashMap<u8, Vec<u8>> ,breaks: &HashMap<u8, HashSet<u8>>, teamsize: &u8) -> (u8, Vec<Vec<String>>) {
    // writes the perfect synergies to a json file
    // teamsize is the number of champions in a team
    // get a list of the unit ids for the champions (no references)
    let ps = champs.keys().combinations((*teamsize).into()).par_bridge().filter(|combination| {
        is_perfect_synergy(combination, traits, champtraits, breaks)
    });
    // initialize the vector of vectors
    // each vector is a team
    let mut teams = Vec::new();
    // convert from parallel iterator
    for p in ps.collect::<Vec<Vec<&u8>>>() {
        // initialize the team
        let mut team = Vec::new();
        // for each unit in the perfect synergy
        for unit in p {
            // add the unit to the team
            team.push(champs.get(unit).unwrap().to_string());
        }
        // add the team to the vector of teams
        teams.push(team);
    }
    return (*teamsize, teams)
}
fn do_all_perfect_synergies(champs: &HashMap<u8, String>, traits: &HashMap<u8, String>, champtraits: &HashMap<u8, Vec<u8>>,breaks: &HashMap<u8, HashSet<u8>>, min_teamsize: &u8, max_teamsize: &u8) -> HashMap<u8, Vec<Vec<String>>> {
    // does perfect synergies for all teamsizes between min_teamsize and max_teamsize
    // returns:
    // a hashmap mapping teamsize to a vector of teams
    //
    // initialize the hashmap
    let mut teams = HashMap::new();
    // for each teamsize
    for teamsize in *min_teamsize..=*max_teamsize {

        let now = Instant::now();
        // do the perfect synergies for the teamsize
        let (teamsize, team) = do_perfect_synergies(champs, traits, champtraits, breaks, &teamsize);
        // add the teams to the hashmap
        teams.insert(teamsize, team);
        // print the time it took
    let elapsed = now.elapsed();
    // print elapsed time
    println!("Finished teamsize {} in {}.{:09} seconds", teamsize, elapsed.as_secs(), elapsed.subsec_nanos());
    }
    // return the hashmap
    teams
}
fn perfect_synergies_to_json(teams: &HashMap<u8, Vec<Vec<String>>>, file: &str) {
    // writes the perfect synergies to a json file using serde_json
    // teams is a hashmap mapping teamsize to a vector of teams
    // file is the name of the file to write to
    let mut file = File::create(file).unwrap();
    let teams_json = serde_json::to_string_pretty(teams).unwrap();
    file.write_all(teams_json.as_bytes()).unwrap();
}

fn main() {
// produce the following
// hashmap champs int, string where the ints are champid and the strings are names
// hashmap traits int, string where the ints are trait ids and the strings are trait names
// hashmap breaks int, HashSet<int> where the ints are trait ids and the set is the set of breakpoints
// champtraits hashmap int, hashset<int> where the ints are champ ids and the set is the set of trait ids for the champ
    let (champs, champs_rev) = read_champs("champs.csv");
    let (traits, traits_rev) = read_traits("traits.csv");
    let breaks = read_breaks("traits.csv", &traits_rev);
    let champ_traits = read_champ_traits("champs.csv", &champs_rev, &traits_rev);
    let teams = do_all_perfect_synergies(&champs, &traits, &champ_traits, &breaks, &1, &9);
    perfect_synergies_to_json(&teams, "perfect_synergies.json");

}
