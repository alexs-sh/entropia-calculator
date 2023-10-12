# About

[![Build status](https://gitlab.com/alexssh/entropia-calculator/badges/master/pipeline.svg)](https://gitlab.com/alexssh/entropia-calculator/-/commits/master) 
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)


Yet another
[entropy](https://en.wikipedia.org/wiki/Entropy_(information_theory))
calculator. It allows to split of the input file into blocks with user-specified
sizes and shows the entropy of each block as std output or PPM picture.


**PPM output example**:

```
cargo run -- --file ./test-file --block-size=16384 --ppm-report=./report.ppm
```
The red color is for blocks with high entropy. The blue color is for low 

![Example1](doc/example1.png)

**Text output example**:

```
cargo run -- --file ./test-file --block-size=16384
--------------------------------------------------------------------------------
Block       Entropy
--------------------------------------------------------------------------------
00000000    2.893
00000001    2.539
00000002    1.938
00000003    6.059
00000004    6.177
00000005    5.846
00000006    6.053
00000007    6.018
00000008    6.094
...
00000303    5.231
00000304    0.250
```





