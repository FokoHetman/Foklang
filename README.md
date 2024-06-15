# FokLang
aka Foko (literally me)'s language inspired by haskell, nix and (in the future) perl. Mostly was inspired to make it because configs (why use universal stuff like json when i can have my own thing) and well... having your own language is,,, kinda cool I think?


TODO: Loops, Conditionals
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
foklang$ println "hello foklang!!!"
hello foklang!!!
```
A bit of terminology:
* String, as any other type is called it's Latin word: `Filum`, however we're not using it here. <br>
* String in foklang, just like in haskell, is a list of `Ustulo` (chars, such as: `'h'`). So `"hello"` == `['h'; 'e'; 'l'; 'l'; 'o']`.<br>
<br>good i guess? Let's get to the cool stuff
```
foklang$ let fn x y = x+y
```
what could that do? <br>Exactly! It defines a Moenus (function) that takes arguments (x,y), and adds them together.
<br>Let's see it in action
```
foklang$ fn 2 4
6
```
neat, but why not start with variables?
Well, variables are basically "argumentless" functions
```
foklang$ let ScaleOfConfusionNathanObtainsTheMomentIToldHimThat = 642
foklang$ ScaleOfConfusionNathanObtainsTheMomentIToldHimThat + 5
647
```
alright alright. What else do we have?
Lists, or if you prefer, `Inventarii`. 
These are pretty trivial:
```
foklang$ [1 2 3 4 5 6]
1;2;3;4;5;6;
```
huh? `;` is a default separator, however (unless you use identifiers, `x`, `variable` etc.) aren't needed
```
foklang$ let x = 5
foklang$ let list = [1 2 3 4 x; 5]
foklang$ get list 3
4
```
see? Separator (`;`) goes after identifier (otherwise identifier takes everything after it as argument).<br>
now, builtin function: `get`. 1st argument is a list (in that case, variable `list`). 2nd argument is index. Index starts from 0, so:
```
foklang$ get list 0
1
foklang$ gett list 4
5
```
etc.

Now, the last thing I got to offer you are Objects (or Configs, debatable).
```
foklang$ let var = 5
foklang$ let config = {enable = true offset = 16 variable = var; powers = 12}
foklang$
```
Notice how separators work the same way as in lists.<br>
Now that we got ourselves a config, let's use it!
```
foklang$ get config enable
true
foklang$ get config variable
5
foklang get config powers
12
```
neat. NOW WAIT FOR CONDITIONALS AND LOOPS BECAUSE I STILL DON'T HAVE THEM AAAAAAAAAAAAAAAAAAAAAAAAA
