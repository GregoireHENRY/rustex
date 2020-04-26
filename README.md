# rustex
Create pdf and beamer presentation like in latex. Build your own setup to quickly create your new presentations.

## Build
```
./compile $ARG
xdg-open main.pdf
```

`$ARG`: default(debug), release

## Usage
`.env` contains:
* dimensions in mm
* font from `rsc/font/` (only Arial and RobotoMono-Regular currently set for centering [see `src/rustex.rs` `Doc.magicx`])

Slides located in `rsc/slide/`. Title page example provided.

## Todo
* header / footer
* other slides
* TOC slide
