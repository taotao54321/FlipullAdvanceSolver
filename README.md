# NES Flipull (v1.0) advance mode solver

## Extract a problem from the game ROM

```sh
$ cargo run --example=extract_problem -- Flipull.nes 1
```

## Solve a problem

```sh
$ cargo run --example=solve --release -- problem/01.in
```

## Convert a solution to a NESHawk movie fragment (you can paste it to TAStudio)

```sh
$ cargo run --example=format_solution -- --format=neshawk problem/01.in problem/01.out
```
