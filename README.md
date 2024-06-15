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
```sh
git clone https://github.com/fokohetman/foklang-temp
rustc shell.rs
./shell
```
> *requires git and rustc

# Usage
Upon running shell, you will find yourself in foklang environment:
```
foklang$ 
```

alright, time for... hello world I guess?
```
foklang$ println "hello world"
hello world
```
cool i guess? Let's get to the cool stuff
```
foklang$ let fn x y = x+y
```
what could that do? Exactly! It defines a function that takes arguments (x,y), and adds them together.
Let's see it in action
```
foklang$ fn 2 4
6
```
neat
