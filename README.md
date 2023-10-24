# french-num
A french number exercise written in Rust.  This project uses the online-downloader script in https://github.com/cloudgen2/online-installer
## Update

### Version v0.2.16 
 * Add online installer

## Online Installation
```
curl -fsSL https://dl.leolio.page/french-num/ | python3
```
For instance, if you are using mac, the download address should be:
```
https://dl.leolio.page/french-num/aarch64-clang/0.2/french-num.tar.gz
```

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
# french-num v.0.4.0
# Mis à jour le: 2023-10-14
#
############################

Tapez 'exit' pour terminer le programme!

== Premier niveau L1 ==
1) Quel est le numéro 7 en français? sept
» C'est correct!
2) Quel est le numéro 4 en français? quatre
» C'est correct!
3) Quel est le numéro 2 en français? deux
» C'est correct!
4) Quel est le numéro 7 en français? sept
» C'est correct!
5) Quel est le numéro 8 en français? huit
» C'est correct!
6) Quel est le numéro 4 en français? quatre
» C'est correct!
7) Quel est le numéro 3 en français? trois
» C'est correct!
8) Quel est le numéro 8 en français? huit
» C'est correct!
9) Quel est le numéro 18 en français? dix-huit
» C'est correct!
10) Quel est le numéro 18 en français? dix-hit
» 'dix-hit'.
» La bonne réponse est 'dix-huit'.
 ** 10) Quel est le numéro 18 en français? dix-huit
» C'est correct!
11) Quel est le numéro 20 en français? vingt
» C'est correct!
12) Quel est le numéro 13 en français? treize
» C'est correct!
13) Quel est le numéro 15 en français? quinze
» C'est correct!
14) Quel est le numéro 14 en français? quatorze
» C'est correct!
15) Quel est le numéro 11 en français? onze
» C'est correct!
16) Quel est le numéro 5 en français? cinq
» C'est correct!
17) Quel est le numéro 6 en français? six
» C'est correct!
18) Quel est le numéro 2 en français? deux
» C'est correct!
19) Quel est le numéro 14 en français? quatorze
» C'est correct!
20) Quel est le numéro 11 en français? onze
» C'est correct!
== Niveau 2 L2 ==
21) Qu'est-ce que c'est? ( 1🍎 )? Ceci est une pomme.
» C'est correct!
22) Qu'est-ce que c'est? ( 10🍎 )? 
```

Happy Programming!

