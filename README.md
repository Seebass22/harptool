harptool
--------
Harptool is a simple CLI tool that prints note layouts for diatonic harmonicas in a variety of tunings, in any key.
You can easily add your own tunings if you're missing any.
```
$ harptool --key G --tuning paddy_richter
overblows            A#  D#  G   A#  C#  F                   
blow bends full step                                     F   
blow bends half step                             A#  C#  F#  
blow                 G   B   E   G   B   D   G   B   D   G   
                     1   2   3   4   5   6   7   8   9   10
draw                 A   D   F#  A   C   E   F#  A   C   E   
bends half step      G#  C#  F   G#      D#                  
bends full step          C                                   
bends 1 1/2 step                                             
overdraws                                    G#  C   D#  G#  

```
Harptool defaults to richter tuning and key of C if you do not specify.

### features
print scale degrees instead of note names
```sh
harptool --degrees
```
specify harmonica key
```sh
harptool --key Bb
```
specify a different tuning
```sh
harptool --tuning wilde
```
colorize notes belonging to a scale
```sh
harptool --scale minor_pentatonic
```
specify a different [position](https://en.wikipedia.org/wiki/Harmonica_techniques#Positions) (1st position is default). Affects scale notes (`--scale`) and scale degrees (`--degrees`).
```sh
# highlight the 2nd position blues scale (C harp played in G)
harptool --scale blues --position 2
```

### adding a custom tuning
Simply add a file in `~/.config/harptool/` with the note layout for your tuning.
Separate notes by spaces, use capitals for notes and do not mix sharps and flats.
You can use any key for this (it does not need to be C).

example for natural minor tuning:
```
C Eb G C Eb G C Eb G C
D G Bb D F A Bb D F A
```
