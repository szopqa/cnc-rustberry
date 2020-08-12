# cnc-rustberry
CNC raspberry controller implemented in Rust


## G-code interpreter

Utility used to parse G-code NIST RS274NGC standard: https://www.nist.gov/publications/nist-rs274ngc-interpreter-version-3?pub_id=823374

### Required commands v.1:

|                                     Command                                     | Meaning                                                                                       |
|:-------------------------------------------------------------------------------:|:---------------------------------------------------------------------------------------------:|
| G00 [X(number)] [Y(number)] [F(number)] G01 [X(number)] [Y(number)] [F(number)] | Absolute mode: Move in a line to (X,Y) at speed F Relative Mode: Move (X,Y) amount at speed F |
| G04 P(number)                                                                   | Do nothing for P seconds                                                                      |
| G90                                                                             | absolute mode                                                                                 |
| G91                                                                             | relative mode                                                                                 |
| G92 [X(number)] [Y(number)]                                                     | change logical position                                                                       |