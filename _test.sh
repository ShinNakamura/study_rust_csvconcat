#! /bin/bash
unalias -a

test -d ./sample || mkdir ./sample
cargo run -- ./tests/members*.csv >./sample/one.csv
diff ./tests/expect.csv ./sample/one.csv
