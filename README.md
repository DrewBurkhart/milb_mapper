# MILB Mapper

> A simple app to determine the distance between Minor League Affiliates for each
> major league club.

## Install

```sh
cargo build
```

## Usage

```sh
cargo run "[search query]"
```

## Goals
1. Take in club name
2. Get affiliates (Name, Level, City, State) from CSV
3. Create a vec of distances for each affiliate to each other affiliate
    (i.e. with 5 affiliates there will be 5 vecs with 4 values each)
4. Get an average for each affiliates distances
5. Get an average for all affiliates