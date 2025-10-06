# fview - its like `ls` and `eza` and `exa` but worse

fview (file-view, clever name huh) is a  small utility tool that can quickly list the files in a given directory.
We show the name, creation date (localized to the system), permissions in UNIX rwx format and an icon too!

## why?

Why not.

## No really, why?

I just want to git gudder at Rust so it's more important to just make stuff and have fun.
This is mostly a hobby project to see if I can make a project, that has more use than just
copy pasting random snippets and hoping it works. 
(like the `rmv` project even though that sometimes just fails for no reason or just deletes 
files it shouldnt even touch)

## usage

clone the repo, navigate to the folder and run `cargo install --path .` to install fview.

then to use it just run `fview` in whatever folder you are in and it will show the defaults.

for help use `fview --help`.

Easy use:
`fview -C -d=3`

this means fview, show me the current directory,
canonicalize the file paths `-C` and show me a max depth of 3 `-d=3`

if you do `fview ~ -C` this will show the home dir ofcourse etc etc. you know how `ls` works


