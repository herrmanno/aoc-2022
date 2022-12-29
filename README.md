# Advent of Code 2022

| Day    | Parse    | Part 1   | Part 2   | Total    |
| :----- | -------: | -------: | -------: | -------: |
|  Total |      4ms |    123ms |    550ms |    678ms |
|      1 |    181μs |      4μs |     27μs |    240μs |
|      2 |     70μs |     20μs |     17μs |    116μs |
|      3 |    173μs |     44μs |     50μs |    275μs |
|      4 |    146μs |     10μs |      7μs |    171μs |
|      5 |    270μs |     36μs |    132μs |    453μs |
|      6 |      9μs |     10μs |     53μs |     80μs |
|      7 |    176μs |    189μs |    225ns |    373μs |
|      8 |     86μs |    215μs |    450μs |    761μs |
|      9 |      6μs |    528μs |    630μs |      1ms |
|     10 |     14μs |      4μs |      1μs |     73μs |
|     11 |     23μs |     34μs |     12ms |     12ms |
|     12 |     64μs |    248μs |    194μs |    519μs |
|     13 |    680μs |     21μs |     57μs |    769μs |
|     14 |     98μs |     97μs |      1ms |      1ms |
|     15 |     20μs |      6μs |     35μs |     69μs |
|     16 |    280μs |      9ms |     32ms |     42ms |
|     17 |     72μs |      3ms |      3ms |      6ms |
|     18 |    399μs |    148μs |      1ms |      2ms |
|     19 |     35μs |      9ms |      8ms |     17ms |
|     20 |    222μs |      4ms |     68ms |     73ms |
|     21 |    975μs |    110μs |    239μs |      1ms |
|     22 |    473μs |    147μs |    241μs |    871μs |
|     23 |     81μs |      2ms |    179ms |    181ms |
|     24 |     93μs |     92ms |    241ms |    333ms |
|     25 |     28μs |     56μs |     23ns |     90μs |

## Build
```sh
cargo build [--release]
```

## Run

### Run single part
```sh
./target/{debug,release}/aoc2022 [day] [part] < input.txt
```

### Run single day
```sh
./target/{debug,release}/aoc2022 [day] < input.txt
```

### Run all days

Input files will be read from `./input` directory.

```sh
./target/{debug,release}/aoc2022
```