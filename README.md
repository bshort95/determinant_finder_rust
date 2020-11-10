# determinant_finder_rust
a program that solves two_by_two and three_by_three matrix

## intro

An important part of understanding a matrix is determining if it invertable, this proccess is simple enough but it can become tedious real fast,  
this is what inspired my to write this application. Simply put this program can calculate the determinant of a 3x3 and a 2x2 matrix,  
and as long as it is not equal to 0 then the matrix is considered invertable.  

## setting up the enviorment  
  
this program was written in rust, this is the first program I ever wrote in rust and there definitly are some different syntax tropes that I had to get used to,  
but first things first, 

to set up the enviorment I had to download rust from its website (listed below)  

after it downloads a .exe file there will be a small console that opens up,  
and it will prompt you to either install rust with the default settings,(which is what I did) or to customize them. 

after download you create your first program by opening your command prompt  
and navigate to the file you want your program in. when you get there you type  

cargo new "name of the program you want"  

and then rust will create a new folder system for you with a source file and a couple other system required files.

I used v.s. code for my IDE, and I just had to download the rust extention to use this program. 

to test your program you open up your terminal and after saving your file you type

cargo run

and it will build compile and run your program

## how i built my program

the program was built to demonstrate how to use

* user input
* variables
* loops
* functions
* built in rust arithmatic
* arrays

this was accomplished by using two functions. one that can solve the determinant of a two by two matrix

and another to use the first function mentioned to solve a three by three matrix.
solving a four by four matrix is possible to code it but it is a lot more complicated in the long run

## my program in action

when you first start the program you will be given three options


![instructions example](https://github.com/bshort95/determinant_finder_rust/blob/master/src/Capture1.JPG?raw=true) 

if you choose a number outside range or you write a random letter
you will be prompted to make another choice

![instructions example](https://github.com/bshort95/determinant_finder_rust/blob/master/src/Capture1.1.JPG?raw=true)

if you press one you will be asked what the content of your matrix is one number at a time, when entered the program imediatly produces the answer

![instructions example](https://github.com/bshort95/determinant_finder_rust/blob/master/src/Capture2.JPG?raw=true)

the three by three matrix option works the same with the exception that you need to type in all nine numbers in instead

![instructions example](https://github.com/bshort95/determinant_finder_rust/blob/master/src/Capture3.JPG?raw=true)

when it is all said and done you can press 3 to quit, and it will send you off with a good bye

![instructions example](https://github.com/bshort95/determinant_finder_rust/blob/master/src/Capture4.JPG?raw=true)

## useful websites
[Git](https://git-scm.com/)  
[visual studios code](https://code.visualstudio.com/)  
[rust website](https://www.rust-lang.org/)  
[youtube](https://www.youtube.com/)  
[determinant info](https://en.wikipedia.org/wiki/Determinant)

