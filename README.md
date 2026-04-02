## Overview

This project explores the structure of solution paths in the Rubik’s Cube group.

The goal of this program is to replicate a game I played with my friends at school:

I would ask you for a number N, then I would take a Rubik's cube and make N moves on it.
Then, you would be tasked with solving the cube.
***But***, the challenge comes from the fact that you cannot a traditional algorithmic solution, you must try to reverse the N moves that I made on the cube.

But, what if you don't have any friends?
The problem with playing this game alone is that by scrambling the cube yourself, you already know the solution. 

While I cannot make a program to generate you friends, I can solve the problem of playing this game alone.

I have created a Rubik's cube using modified version of the IDA* algorithm.
This solver finds an alternate path to a scramble that has a solution of length N.
By doing this, the user can be presented with a scramble that does not reveal anything about the solution.
For a more detailed and mathematically rigorous description, [Read the full paper](paper/paper.pdf)
