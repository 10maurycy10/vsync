# vsync

A simple vsync tester.

## how it works

this program alternates frames several colors that should combine into gray. 
see https://en.wikipedia.org/wiki/Persistence_of_vision and https://en.wikipedia.org/wiki/Additive_color

## cycle size

the cycle size can be specifyed with the ``-f`` option:

```
    cargo run -- -f 3
```

2 frame : alternates #FF0000 and #00FFFF this should result in a gray window. (more relable, default)

3 frame: alternates #FF0000 and #00FFFF this should result in a gray window. (double frame drops)

## reqest vsync

by default the program will reqest vsync with ``present_vsync()``.

this can be disabled with `` -n ``