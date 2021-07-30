harptool
--------
Harptool is a simple CLI tool that prints note layouts for diatonic harmonicas in a variety of tunings, in any key.
You can easily add your own tunings if you're missing any.
```
$ harptool --key G --tuning paddy_richter
blow bends full step                                     F   
blow bends half step                             A#  C#  F#  
blow                 G   B   E   G   B   D   G   B   D   G   
                     1   2   3   4   5   6   7   8   9   10
draw                 A   D   F#  A   C   E   F#  A   C   E   
bends half step      G#  C#  F   G#      D#                  
bends full step          C                                   
bends 1 1/2 step                                             
```
Harptool defaults to richter tuning and key of C if you do not specify.

### adding a custom tuning
Simply add a file in `~/.config/harptool/` with the note layout for your tuning.
Separate notes by spaces, use capitals for notes and do not mix sharps and flats.
You can use any key for this (it does not need to be C).

example for natural minor tuning:
```
C Eb G C Eb G C Eb G C
D G Bb D F A Bb D F A
```
