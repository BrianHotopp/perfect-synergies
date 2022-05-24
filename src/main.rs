// hashmap and hashset
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
// Combinations
use itertools::Itertools;
use rayon::prelude::*;
// serde json
use serde_json::json;
// serialize
use serde::{Deserialize, Serialize, Serializer};
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
fn read_costs(_file: &str, champs_rev: &HashMap<String, u8>) -> HashMap<u8, u8> {
    // reads the cost of each champion from the _file
    // returns:
    // a hashmap mapping integers to the cost of the champions
    let mut costs = HashMap::new();
    let contents = fs::read_to_string(_file).unwrap();
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        // each line has the form
        // <champion name>, <cost>, <trait 1>, <trait 2> ... <trait n>
        // we just want to extract the cost
        let mut line = line.split(",");
        let name = line.next().unwrap().trim();
        let cost = line.next().unwrap().trim().parse::<u8>().unwrap();
        costs.insert(champs_rev[name], cost);
    }
    costs
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
fn less_than_n_wasted(team: &Vec<&u8>, traits: &HashMap<u8, String>, unit_traits: &HashMap<u8, Vec<u8>>,wastes: &HashMap<u8, HashMap<u8, u8>>, n: &u8) -> bool {
    // checks if the team has less than n wasted traits
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
    let mut waste_count = 0;
    for (trait_id, count) in team_traits.iter_mut() {
        // for each trait in the team get the waste for the count
        waste_count += wastes[trait_id][count];
    }
    if waste_count <= *n {
        return true;
    }
    false
}
// serde serialization
#[derive(Serialize, Deserialize)]
struct Team {
    size: u8,
    // a team is a vector of champions
    team: Vec<String>,
    // the active traits of the team and their level
    active_traits: HashMap<String, u8>,
    // the traits wasted by the team 
    wasted_traits: HashMap<String, u8>,
    // total traits wasted by the team
    total_wasted_traits: u8,
    // the total cost of the team
    total_cost: u8,
    // the highest cost unit in the team (tuple of name and cost)
    max_cost: (String, u8),
    // the lowest cost unit in the team (tuple of name and cost)
    min_cost: (String, u8),
    // the average cost of the team
    average_cost: f64,
}

impl Team {
    fn new(team: Vec<String>, size: u8, active_traits: HashMap<String, u8>, wasted_traits: HashMap<String, u8>, total_wasted_traits: u8, total_cost: u8, max_cost: (String, u8), min_cost: (String, u8), average_cost: f64) -> Team {
        Team {
            team,
            size,
            active_traits,
            wasted_traits,
            total_wasted_traits,
            total_cost,
            max_cost,
            min_cost,
            average_cost,
        }
    }
    fn get_team_traits(active_traits: &mut HashMap<String, u8>, wasted_traits: &mut HashMap<String, u8>, total_waste: &mut u8, team: &Vec<&u8>, traits: &HashMap<u8, String>, unit_traits: &HashMap<u8, Vec<u8>>, wastes: &HashMap<u8, HashMap<u8, u8>>) -> u8 {
        let mut team_traits = HashMap::new();
        for trait_id in traits.keys() {
            team_traits.insert(*trait_id, 0);
        }
        for nit in team {
            let unit_traits = &unit_traits[*nit];
            for trait_id in unit_traits {
                team_traits.insert(*trait_id, team_traits[trait_id] + 1);
            }
        }
        let mut waste_count = 0;
        for (trait_id, count) in team_traits.iter_mut() {
            let this_waste = wastes[trait_id][count];
            waste_count += this_waste;
            if this_waste > 0 {
                wasted_traits.insert(traits[trait_id].clone(), this_waste);
            }
            if *count != this_waste || (count > &mut 0 && this_waste == 0) {
                active_traits.insert(traits[trait_id].clone(), *count - this_waste);
            }
        }
        *total_waste = waste_count;
        return 0;
    }
    fn get_team_costs(total_cost: &mut u8, max_cost: &mut (String, u8), min_cost: &mut (String, u8), average_cost: &mut f64, team: &Vec<&u8>, champs: &HashMap<u8, String>, costs: &HashMap<u8, u8>) -> u8 {
        // get the cost of the team
        // return the total cost
        let mut team_cost = 0;
        for nit in team {
            // for each unit in the team
            // get the cost of the unit
            let cost = costs[*nit];
            // add the cost to the team cost
            team_cost += cost;
            // if the cost is greater than the max cost
            if cost > max_cost.1 {
                // record the new max cost
                max_cost.0 = champs[*nit].to_string();
                max_cost.1 = cost;
            }
            // if the cost is less than the min cost
            if cost < min_cost.1 {
                // record the new min cost
                min_cost.0 = champs[*nit].to_string();
                min_cost.1 = cost;
            }
        }
        // record the average cost and team cost
        *average_cost = team_cost as f64 / team.len() as f64;
        
        *total_cost = team_cost;
        // return success
        return 0;
    }
    fn team_from_list(team: &Vec<&u8>, traits: &HashMap<u8, String>,  champs: &HashMap<u8, String>, unit_traits: &HashMap<u8, Vec<u8>>, wastes: &HashMap<u8, HashMap<u8, u8>>, costs: &HashMap<u8, u8>) -> Team {
        let size = team.len() as u8;
        let mut str_team = team.iter().map(|&nit| champs[nit].to_string()).collect::<Vec<String>>();
        let mut active_traits = HashMap::new();
        let mut wasted_traits = HashMap::new();
        let mut total_wasted_traits = 0;

        Team::get_team_traits(&mut active_traits, &mut wasted_traits, &mut total_wasted_traits, &team, &traits, &unit_traits, &wastes);
        let mut total_cost = 0;
        let mut max_cost = ("".to_string(), 0);
        let mut min_cost = ("".to_string(), 6);
        let mut average_cost = 0.0;
        Team::get_team_costs(&mut total_cost, &mut max_cost, &mut min_cost, &mut average_cost, &team, &champs, &costs);
        return Team::new(str_team, size, active_traits, wasted_traits, total_wasted_traits, total_cost, max_cost, min_cost, average_cost);
    }
}


fn do_ltn_synergies(champs: &HashMap<u8, String>, traits: &HashMap<u8, String>, unit_traits: &HashMap<u8, Vec<u8>>, wastes: &HashMap<u8, HashMap<u8, u8>>, costs: &HashMap<u8, u8>, teamsize: &u8, n: &u8) -> Vec<Team> {
    // returns:
    // a tuple of the following:
    // 1 teamsize, 2 a vector of the teams of that size that have less than n wasted traits

    // initialize the vector of teams
    let teams: Vec<Team> = Vec::new();
    // .par_bridge()
    let ps = champs.keys().combinations((*teamsize).into()).par_bridge().filter({
        |team|
        less_than_n_wasted(team, traits, unit_traits, wastes, n)
    }).map({
        |team|
        Team::team_from_list(&team, &traits, &champs, &unit_traits, &wastes, &costs)
    }).collect::<Vec<Team>>();
    return ps;
}

fn do_all_ltn_synergies(champs: &HashMap<u8, String>, traits: &HashMap<u8, String>, champtraits: &HashMap<u8, Vec<u8>>,wastes: &HashMap<u8, HashMap<u8, u8>>, costs: &HashMap<u8, u8>, min_teamsize: &u8, max_teamsize: &u8, n: &u8) -> Vec<Team> {
    // does less than n wasted synergies for all teamsizes between min_teamsize and max_teamsize
    // returns:
    // a hashmap mapping teamsize to a vector of teams
    // n is the maximum number of wasted traits
    let mut teams = Vec::new();
    // for each teamsize
    for teamsize in *min_teamsize..=*max_teamsize {
        let now = Instant::now();
        // do the ltn synergies for the teamsize
        let ps = do_ltn_synergies(&champs, &traits, &champtraits, &wastes, &costs, &teamsize, &n);
        // extend the list with the new teams
        teams.extend(ps);
        // print the time it took
        let elapsed = now.elapsed();
        // print elapsed time
        println!("Finished teamsize {} in {}.{:09} seconds", teamsize, elapsed.as_secs(), elapsed.subsec_nanos());
    }
    return teams;
}
fn synergies_to_json(teams: &Vec<Team>, file: &str) {
    // writes   
    let mut file = File::create(file).unwrap();
    // write the json with indents
    let json_teams = json!(teams);
    // pretty string
    let pretty_json = serde_json::to_string_pretty(&json_teams).unwrap();
    // write the string to the file
    file.write_all(pretty_json.as_bytes()).unwrap();
}
fn compute_wastes(breaks: &HashMap<u8, HashSet<u8>>) -> HashMap<u8, HashMap<u8, u8>> {
    // returns:
    // a hashmap mapping a trait to the wasted traits for each count of that trait

    let mut wastes = HashMap::new();
    // for each trait
    for trait_ in breaks.keys() {
        // initialize the wasted traits for that trait
        let mut wasted_traits = HashMap::new();
        // for each possible number of that trait from  1 to 9 (overkill but whatever)
        for count in 0..=9 {
            // waste for a count is count-(largest number in the breaks hashset <= count)
            wasted_traits.insert(count, count - breaks.get(trait_).unwrap().iter().filter(|x| **x <= count).max().unwrap());
        }
        // add the wasted traits to the hashmap
        wastes.insert(*trait_, wasted_traits);
}
    wastes
}
fn main() {
// produce the following
// hashmap champs int, string where the ints are champid and the strings are names
// hashmap costs string, int where the strings are champ names and the ints are the costs
// hashmap traits int, string where the ints are trait ids and the strings are trait names
// hashmap breaks int, HashSet<int> where the ints are trait ids and the set is the set of breakpoints
// champtraits hashmap int, hashset<int> where the ints are champ ids and the set is the set of trait ids for the champ
    let (champs, champs_rev) = read_champs("champs.csv");
    let costs = read_costs("champs.csv", &champs_rev);
    let (traits, traits_rev) = read_traits("traits.csv");
    let breaks = read_breaks("traits.csv", &traits_rev);
    let champ_traits = read_champ_traits("champs.csv", &champs_rev, &traits_rev);
    let wastes = compute_wastes(&breaks);
    let n = 3;
    let min_team_size = 3;
    let max_team_size = 8;
    let teams = do_all_ltn_synergies(&champs, &traits, &champ_traits, &wastes, &costs, &min_team_size, &max_team_size, &n);
    let fname = format!("ltn_synergies_waste_size_{}_to_{}_maxloss_{}.json", min_team_size, max_team_size, n);
    //let teams = do_all_perfect_synergies(&champs, &traits, &champ_traits, &breaks, &1, &4);
    //let fname = format!("perfect_synergies.json");
    synergies_to_json(&teams, &fname);
}
