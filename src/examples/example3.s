INIT 256
CLOAD 0
STORE 2
CLOAD 1
STORE 3
LOAD 3
SUB 1
IF c(0) > 0 THEN GOTO 15
LOAD 2
CADD 1
STORE 2
LOAD 3
CMULT 2
STORE 3
JMP 4
LOAD 2
CSUB 1
STORE 1
END