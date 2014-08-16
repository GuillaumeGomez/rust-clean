rust-clean
==========

A little temporary emacs file cleaner in rust. It removes all the ~ files (file.txt~ for example). Not the #files#.

##Installation

To build it, please use :

```Shell
> make
```

If you want to directly install this software in your environment, please use :

```Shell
> make install
```

##Available options

For the moment, here are the available options :

 * -r : recursive mode
 * -v : verbose mode
 * -i : prompt before every removal
 * --help : print this help


##Short example

```Shell
> clean . ~/Downloads -v
```

##License

    Copyright (c) 2014 Guillaume Gomez
    
    This software is provided under the Zlib/png license.
