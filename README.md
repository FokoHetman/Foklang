# FokLang
aka Foko (literally me)'s language inspired by haskell, nix and (in the future) perl. Mostly was inspired to make it because configs (why use universal stuff like json when i can have my own thing) and well... having your own language is,,, kinda cool I think?

# Installation
### For nix users:
* shell:
```nix
nix run gitub:fokohetman/foklang-temp
```
* future options...
### For others:
* shell:
*requires git and rustc*
```git clone https://github.com/fokohetman/foklang-temp
rustc shell.rs
./shell
```

# Usage
Upon running shell, you will find yourself in foklang environment:
```
foklang$ 
```
let's write a simple function (this is a functional language after all):
```
foklang$ let addtwo x = x+2
foklang$ addtwo 2
foklang$
``` Ima finish this later because gotta make shell actually not print Proventus, and the Fructa instead