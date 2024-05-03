# Keyboard Splitter

Simple VST3 plugin that gets a MIDI input and split value (parameter) and send all notes with a value below the split value to the MIDI Channel 1 and all notes with a value above the split value to the MIDI Channel 2.

## Why?
Because sometimes you want to use your MIDI keyboard to play two instruments, one with your left hand and one with your right. :3

## Can I split into more than two? Can I X? Can I Y?
Please, use something better like MIDI Polysher (https://www.pluginboutique.com/products/909-MIDI-Polysher) for more complicated scenarios. This one is super simple and only for a single scenario (splitting a single MIDI keyboard to two different instruments), so you can just open a preset and start playing.

## Build Instructions
`cargo xtask bundle keyboard_splitter --release`
