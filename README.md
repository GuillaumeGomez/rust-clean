rust-clean [![Build Status](https://api.travis-ci.org/GuillaumeGomez/rust-clean.png?branch=master)](https://travis-ci.org/GuillaumeGomez/rust-clean)
==========

A little temporary emacs file cleaner in rust. It removes all the ~ files (file.txt~ for example). Not the #files#.

##Installation

To build it, please use :

```Shell
> make
```

Since it supports Cargo, you can also build it this way :

```Shell
> cargo build
```

If you want to directly install this software in your environment, please use :

```Shell
> make install
```

##Available options

For the moment, here are the available options :

 * -r          : recursive mode
 * -v          : verbose mode
 * -i          : prompt before every removal
 * -l=[number] : Add a level for recursive mode
 * --help : print this help


##Short example

```Shell
> clean . ~/Downloads -v
```

##License

    Copyright (c) 2014-2015 Guillaume Gomez
    
    This software is provided under the Zlib/png license.
