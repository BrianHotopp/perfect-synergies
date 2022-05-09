import pandas as pd
def find_best_unit(units):
    # units is a pandas df
    # columns are champ_name,champ_popularity,champ_top_4_percentage,champ_top_1_percentage,average_placement
    # add number of players beaten, so that higher is better
    units['players_beaten'] = 7 - units['average_placement']
    # normalize the columns
    units['champ_popularity'] = units['champ_popularity']/units['champ_popularity'].max()
    units['champ_top_4_percentage'] = units['champ_top_4_percentage']/units['champ_top_4_percentage'].max()
    units['champ_top_1_percentage'] = units['champ_top_1_percentage']/units['champ_top_1_percentage'].max()
    units['average_placement'] = units['average_placement']/units['average_placement'].max()
    units['players_beaten'] = units['players_beaten']/units['players_beaten'].max()
    # perform min-max feature scaling
    units['champ_popularity'] = (units['champ_popularity'] - units['champ_popularity'].min())/(units['champ_popularity'].max() - units['champ_popularity'].min())
    units['champ_top_4_percentage'] = (units['champ_top_4_percentage'] - units['champ_top_4_percentage'].min())/(units['champ_top_4_percentage'].max() - units['champ_top_4_percentage'].min())
    units['champ_top_1_percentage'] = (units['champ_top_1_percentage'] - units['champ_top_1_percentage'].min())/(units['champ_top_1_percentage'].max() - units['champ_top_1_percentage'].min())
    units['average_placement'] = (units['average_placement'] - units['average_placement'].min())/(units['average_placement'].max() - units['average_placement'].min())
    units['players_beaten'] = (units['players_beaten'] - units['players_beaten'].min())/(units['players_beaten'].max() - units['players_beaten'].min())
    # compute composite score, as the sum of popularity and players beaten
    units['composite_score'] = units['champ_popularity'] + units['players_beaten']
    # sort and return
    return units.sort_values(by='composite_score', ascending=False)
def read_data(file_name):
    # file_name is a string
    # returns a pandas df
    # columns are champ_name,champ_popularity,champ_top_4_percentage,champ_top_1_percentage,average_placement
    units = pd.read_csv(file_name)
    return units

if __name__ == '__main__':
    import sys
    import os
    # get the file name from the command line
    file_name = sys.argv[1]
    # check if the file exists
    if os.path.isfile(file_name):
        # read the data
        units = read_data(file_name)
        # rank the units
        df = find_best_unit(units)
        # print the units and their attributes
        # write the data to a file
        df.to_csv('ranked_units.csv')
    else:
        print('File does not exist')
        sys.exit(1)




    
