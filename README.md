# Asciilines

Copyright (c) 2019 Sharice Mayer
shama2@pdx.edu


## Explanation of what the program is and does

_HW 1: Make a mini-project_  
This is the Mini-Project: HW1 for CS461P-Open Software Development  
This program accepts a single .tvg file argument and renders the file as ASCII on standard output.  
The output canvas specified by the tvg file is output as ASCII on stdout, as well as a .out file


### The TVG Format  

A "TVG" file contains "Text Vector Graphics".  

The first line of the file contains the "canvas size":  
- Two numbers representing total rows and columns of text to output
- Both must be single digits greater than zero.  
- The canvas starts out filled with . characters.

The rest of the file contains rendering commands, one per line.  
A rendering command is a line containing:  
- A character to render onto the canvas  
    -Must be an ascii character value  
- A row position to start at (0-based)  
    -Must be a single digit(<10)  
- A column position to start at (0-based)  
    -Must be a single digit(<10)  
- A letter indicating direction  
    -Either 'h' for a horizontal line or 'v' for a vertical line  
    -Horizontal lines go to the right from the starting coordinate  
    -Vertical lines go down from the starting coordinate  
- A length for the rendered line  
    -Must be greater than 0  

*The elements of the rendering command should be separated by a single space.*  

The character positions that are part of the rendering command's rendered line should be filled with the rendering character. 
It is legal for the line to extend outside the canvas.  
There is no wraparound: only points inside the canvas should be rendered, others should be ignored.  

A rendering output is produced by executing each of the rendering commands on the canvas. 
For example, a TVG file containing:

     6 7
     * 4 -1 h 7
     x 0 -1 v 6
     # -1 3 v 7
     $ -1 0 h 8
     ? 0 4 v 4

should render as:

    $$$$?$$
    x..#?..
    x..#?..
    x..#?..
    x**#.**
    x..#...


## Build and Run  

Build and Run this program and with `cargo run testfilename.tvg` 
passing in a tvg file as the program argument(in Rust)
for example:

cargo run tests/test1.tvg


Here is an example session. 
The lines beginning with $ were typed by me: the other lines are program output.  
Note: The "empty" lines in the example are intentional program output to help visibility.  

    $ cargo run tests/test1.tvg
     
    The filename read in was:
      tests/test1.tvg
      
    Contents of file:
      3 4
      * 1 -1 h 5
      # -1 1 v 5
      
    Successful write!
    Contents of tests/test1.out: 
      .#..
      *#**
      .#..
     
    $ 


*need to fix to add my copy*
To run the python version instead:  

python3 asciilines.py tests/test1.tvg


Here is an example session.  
The lines beginning with $ were typed by Bart Massey: the other lines are program output.  

    $ cat tests/test1.tvg 
    3 4
    * 1 -1 h 5
    # -1 1 v 5
    $ python3 asciilines.py tests/test1.tvg
    .#..
    *#**
    .#..
    $ 


## Bugs, Defects, Failing Tests, etc

Bugs, Defects, and Failing Test information goes here as needed.
The program fails on invalid input, but may panic or otherwise display an error in this case. 


## License

This program is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.  

## Acknowledgments  

Thanks to professor Bart Massey for much of the underlying README information and project guidance.
Thanks to my classmates for their support as well.



