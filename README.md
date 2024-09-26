Creates .dat files of different sizes based on arguments given in the command line.
The first argument is the number of a certain unit that you would like, it is supposed to be a unsigned integer.
The second argument corresponds to the unit, there are only three right now: KB, MB and GB.
The file will be named "size_unit.dat"

## Example
 ```
 cargo run -- 20 KB

```