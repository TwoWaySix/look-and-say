# A Look-And-Say Command Line Tool
A command line tool written in Rust that solves the **Look-and-say sequence** with any given starting number.

    Description on Wikipedia 
    https://en.wikipedia.org/wiki/Look-and-say_sequence

    To generate a member of the sequence from the previous member, 
    read off the digits of the previous member, counting the number 
    of digits in groups of the same digit. 
    For example:
    - 1 is read off as "one 1" or 11.
    - 11 is read off as "two 1s" or 21.
    - 21 is read off as "one 2, then one 1" or 1211.
    - 1211 is read off as "one 1, one 2, then two 1s" or 111221.
    - 111221 is read off as "three 1s, two 2s, then one 1" or 312211.
    The look-and-say sequence was introduced and analyzed by John Conway.

