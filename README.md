# Keyboard Splitter

Simple VST3 plugin that gets a MIDI input and split value (parameter) and send all notes with a value below the split value to the MIDI Channel 1 and all notes with a value above the split value to the MIDI Channel 2.

## Why?
Because sometimes you want to use your MIDI keyboard to play two instruments, one with your left hand and one with your right. :3

## Build Instructions
`cargo xtask bundle keyboard_splitter --release`
