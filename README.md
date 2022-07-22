# jot

**Brutally simple command line app for jotting things down.**

---

## About
**jot** lets you get things down before you forget, 
without fiddling folders, naming, etc., 
letting you write first and think about what to do with it later.
Just open the Terminal and begin your note with the `jot` command.
**jot** is not meant to be a text editor, 
it is just a simple tool to jot down notes in the fastest possible way.
You can open 'jot.txt' later in a text editor by your choice 
and manipulate the collected notes as you wish.

## Getting started
- Build the source code;
- Add the binary file path to PATH;
- Open Terminal and begin your note with the `jot` command;
- The first time the `jot` command is executed 
a file 'jot.txt' is created in the home directory, 
where the data will be written. Each new line in the Terminal that begins 
with the `jot` command is a new line of text in the jot.txt file. 

## Examples
`jot Hello World` writes 'Hello World'

Special characters should be put in quotes or escaped. 
There are several ways to type 'Hello World :)':

`jot "Hello World :)"`

`jot Hello World ":)"`

`jot Hello World :\)`

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)
