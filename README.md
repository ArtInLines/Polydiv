# Polydiv

## Goals

Try to write the same program in different languages and compare the difficulties.

This program should especially push many type systems to their limits and it should be fun and educational how far languages can go.

These are the goals for the program for all languages - if some are too difficult or even impossible to represent like this, then I will simply leave them out of that language:

-   Represent Algebraic Structures (at least Magmas, Groups, Rings and Fields) abstractly
    -   Preferably some function signatures should already be provided for each of these
-   Implement some algorithms for these Algebraic Structures (e.g. GCD)
-   Implement the following types as said Algebraic Structures
    -   Natural Numbers
    -   Integers
    -   Rationals
        -   Should offer simplification of Rationals (requires GCD)
    -   Polynomials
        -   Should be specified on a certain variable
        -   Should take most algebraic structures as coefficients
        -   Different calculations for polynomials on the same variable
    -   Z/Zn Groups
    -   Galois Fields
-   If possible, add support for Rationals of Polynomials or any specific Algebraic Structure
-   If feasible, add support for Vector-Fields and Scalarfields

    -   Add support for Algebras at that point too

-   If feasible, implement Abstract Tokens for each of these types to offer symbolic execution
