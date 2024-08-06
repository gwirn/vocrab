# voc(r)ab 🦀

A terminal vocabulary quiz that tracks your progress, asks you which words you know the least of first, and shows you your 10 worst words after each unit so you can review them.

## Installation
[Install rust and cargo](https://www.rust-lang.org/tools/install)

Run `cargo build --release` to create a binary of the program.

Create an environment variable storing the base path where the vocabulary directory is stored.
`export VOCRAB="/PATH/TO/VOCRAB_VOCABULARY"`

## Vocabulary creation
Create a directory named `vocabulary` in `/PATH/TO/VOCRAB_VOCABULARY`.
In there create a csv file containing the words in your language and the language you want to learn like

```csv
red,rot
blue,blau
yellow,gelb
```

Then run (assuming you have an alias to the binary) `vocrab -i NAME_OF_CSV_UNIT.csv -o NAME.cbor`.

This creates the needed data structure to read the vocabulary and store your progress.
By default the words on the left will be shown to you and you will have to enter the words on the right.

## Commands
```
type :q to quit unit
type :q! to quit unit without saving
type :qa to quit vocrab
type :qa! to quit vocrab without saving
type 'NUMBER'r to reverse question order
```
