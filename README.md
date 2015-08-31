```puzzlegen.cc``` is a simple program in C++14 that generates anagram
puzzles as found in the New York Times Magazine, that they call
"Spelling Bee".  These puzzles present a circle of six letters
around a seventh, central letter, like
```
    M   O
  P   I   S
    T   U
```
The goal is to find common words that use only the letters in the
set, and that all use the central letter.  Words that use all the
letters score extra: one point for each lesser word, and three for
each that uses all seven.  For example, for the letters above,
"mitosis" scores 1, "optimums" 3.  The program only emits puzzles
that that it finds have between 26 and 32 points possible, given
the words in its list.  Typically one should be satisfied to find
20 points' worth.

Output is a list of seven-letter sets, like
```
  $ ./puzzlegen
  ...
  imOprSy
  imOprtu
  Imopstu
  ImoPstv
```
Capital letters in output are candidates for the central letter.

```solve.sh``` is a simpler program that, given such a puzzle, lists
words found in /usr/share/dict/words that solve the puzzle. An
excerpt from its output for the puzzle above is,
```
  $ ./solve.sh imopstu
  ...
  optimum
  optimums *
  osmosis
  pimps
```
with three-point words suffixed " *".  (The central letter comes first
in the command-line argument.)

### Internals

```puzzlegen.cc``` may be more interesting as an example of optimized modern
C++ coding than as a generator of puzzles.  It uses bits in a 32-bit word,
via bitset<>, to represent sets of letters, bitwise arithmetic to step
through the set and qualify words, and new-style for-loops over containers.
It uses an STL-style container adapter over std::bitset<> to provide a
conforming iterator usable with the new-style for-loop.

As important is what it doesn't use.  It doesn't store the actual words it
reads, as they are not useful.  It uses ```<set>```, not ```<unordered_set>```,
because (a) with ```set``` it is *exactly* as fast, but (b) produces more-
pleasingly ordered output.  It makes only one pass through all the candidate
words for each candidate letter-set.  It discards words on input that cannot
be solutions.  Early versions used lambda functions that ended up being
better-placed in the container adapter (although on Haswell that is slower).

It does depend on a runtime character set with contiguous alphabetic
characters, and, by default, a ```/usr/share/dict/words``` file in the right
place.

The C++ version puzzlegen-old.cc runs faster on Intel Haswell than any other
version, while all the C++ versions run about the same speed on Westmere.
I.e., on Haswell most versions run artifically slowly. Curiously, adding a
line "++count;" in the innermost loop makes puzzlegen-int.cc run as fast on
Haswell as puzzlegen-old.cc.

The Rust version of the program runs, on Intel Westmere, about 50% slower
that the C++ version; on Haswell, 25% faster, but as a consequence of bugs
in Haswell hardware, not superiority of Rust code generation.

Alternative versions of the programs differ:

  - puzzlegen.cc     -- base version, uses local bitset_set.h iterators
  - puzzlegen-fix.cc -- iteration controlled by element indices, not bit masks
  - puzzlegen-int.cc -- uses unsigned int rather than std::bitset<26>
  - puzzlegen-min.cc -- uses unsigned int, and avoids lambdas
  - puzzlegen-old.cc -- posted in gcc bug #67153, only version fast on Haswell
  - puzzlegen.rs     -- in Rust, reading into Vec<u8>, iterating by index
  - puzzlegen-sm.rs  -- reading via state machine, iterating by index
