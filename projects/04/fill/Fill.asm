// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, the
// program clears the screen, i.e. writes "white" in every pixel.

// Put your code here.
@INF
0;JMP

(INF)
    @curr
    M=0
    @KBD
    D = M
    @SCREENLOOP-ON
    D;JNE
    @SCREENLOOP-OFF
    0;JMP

(SCREENLOOP-ON)
    @curr
    D = M
    @8192
    D = D-A
    @INF
    D; JGE
    @SCREEN
    D = A
    @curr
    A=D+M
    M=-1
    @curr
    M=M+1
    @SCREENLOOP-ON
    0;JMP

(SCREENLOOP-OFF)
    @curr
    D = M
    @8192
    D = D-A
    @INF
    D; JGE
    @SCREEN
    D = A
    @curr
    A=D+M
    M=0
    @curr
    M=M+1
    @SCREENLOOP-OFF
    0;JMP