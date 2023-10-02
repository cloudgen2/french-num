# french-num
A french number exercise written in Rust

## Update

### Version v0.2.9 
 * Update process_line regex

### Version v0.2.10
 * Added Pear, Cherry and WaterMelon

## Run the source code
```
cargo run
```

## Build release
```
cargo build --release
rm -rf ~/.local/bin/spanish-num
cp target/release/french-num ~/.local/bin/
```

## Execution example

```
############################
#
# french-num v.0.2.10
# Mis à jour le: 2023-07-17
#
############################

Tapez 'exit' pour terminer le programme!

== Premier niveau L1 ==
1) Quel est le numéro 5 en français? cinq
» C'est correct!
2) Quel est le numéro 6 en français? six
» C'est correct!
3) Quel est le numéro 2 en français? duex
» La bonne réponse est 'deux'.
 ** 3) Quel est le numéro 2 en français? deux
» C'est correct!
4) Quel est le numéro 6 en français? siix
» La bonne réponse est 'six'.
 ** 4) Quel est le numéro 6 en français? six
» C'est correct!
5) Quel est le numéro 10 en français? dix
» C'est correct!
6) Quel est le numéro 1 en français? un
» C'est correct!
7) Quel est le numéro 7 en français? sept
» C'est correct!
8) Quel est le numéro 10 en français? dix
» C'est correct!
9) Quel est le numéro 7 en français? sept
» C'est correct!
10) Quel est le numéro 8 en français? huit
» C'est correct!
12) Qu'est-ce que c'est? ( 1🍎 )? Ceci est une pomme.
» C'est correct!
13) Qu'est-ce que c'est? ( 9🍎 )? Ce sont neuf pommes.
» C'est correct!
14) Qu'est-ce que c'est? ( 9🍎 )? Ce sont neuf pommes.
» C'est correct!
15) Qu'est-ce que c'est? ( 4🍎 )? Ce sont quatre pommes.
» C'est correct!
17) Qu'est-ce que c'est? ( 1🍊 )? Ceci est une orange.
» C'est correct!
18) Qu'est-ce que c'est? ( 2🍊 )? Ce sont deux oranges.
```

Happy Programming!