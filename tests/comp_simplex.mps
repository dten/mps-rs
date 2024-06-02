* This a 3 by 4 sample MPS file created 2002.03.15
NAME          Sample Problem
ROWS
 N  OBJ              * Objective function
 L  Res-1
 L  Res-2
 L  Res-3
 E  Balance
COLUMNS
    Vol--1    OBJ                4.5   Res-1              1.0
    Vol--1    Balance            2.5
    Vol--2    OBJ                2.5   Res-2              1.5
    Vol--2    Balance            2.0
    Vol--3    OBJ                4.0   Res-1              1.0
    Vol--3    Res-2              0.5   Res-3              3.0
    Vol--4    OBJ                4.0   Res-1              1.5
    Vol--4    Res-2              0.5   Res-3              2.0
RHS
    RHS-1     Res-1             40.0   Res-2             30.0
    RHS-1     Balance           95.0
RANGES
    RANGE-1   Res-2             10.0
BOUNDS
 LO BOUND-1   Vol--3           -10.0
 UP BOUND-1   Vol--3            20.0
 UP BOUND-1   Vol--4            25.0
ENDATA