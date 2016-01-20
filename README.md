rust-clean [![Build Status](https://api.travis-ci.org/GuillaumeGomez/rust-clean.png?branch=master)](https://travis-ci.org/GuillaumeGomez/rust-clean)
==========

A temporary emacs file cleaner in rust. It removes all the ~ files (file.txt~ for example), not the #files#.

##Installation

To build it, please use :

```Shell
> cargo build
```

If you want to directly install this software in your environment, please use :

```Shell
> cargo install
```

##Available options

Here are the available options :

 * -r          : recursive mode
 * -v          : verbose mode
 * -i          : prompt before every removal
 * -l=[number] : Add a level for recursive mode
 * --help : print this help


##Short example

```Shell
> clean . ~/Downloads -v
```
