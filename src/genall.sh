#!/bin/bash
# calls cargo run --release with different parameters to write output files
# parameters are max_loss, min_teamsize, max_teamsize
# max loss ranges from 0 to 4
# min teamsize is fixed at 2
# max teamsize ranges from 6-9

for max_loss in {0..4}; do
    for max_teamsize in {6..9}; do
    # start timer
    start=$(date +%s.%N)
        echo "Running with max_loss = $max_loss, max_teamsize = $max_teamsize"
        cargo run --release out_data $max_loss 2 $max_teamsize
    # end timer
    end=$(date +%s.%N)
    runtime=$(echo "$end - $start" | bc) # calculate runtime
    echo "Runtime: $runtime"
    done
done