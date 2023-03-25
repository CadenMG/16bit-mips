# General Information
Word size:
16 bits

Number of registers:
8


# Control Lines
- BranchOrJumpOrSequential: Based on the type instr. we will update the PC appropriately
- MemAddrFromPCOrALU: Whether to use the PC or the new address from the ALU
- MemReadOrWrite: Whether the memory access is a read or write
- LoadIR: Indicates to load from the IR
- RegReadOrWrite: Whether we are reading or writing to the register array
- SecondOpComingFromImmediateOrReg2: Whether the second argument to the ALU is an immediate or from a register
- WriteRegFromMemoryOrALU: Write to the register from memory or the result of the ALU
- ALUFunction: Indicates the type of operation for the ALU to perform


# ALU Status Lines
- Zero: Whether the ALU procuded 0 as an output
- Negative: Whether the ALU procuded a negative output
- Carry_{out}: Any carry output from the operation
- Carry_{in}: Any carry input for the subsequent operation
- Overflow: Whether an overflow occurred during the operation


# Registers
```
--------------------------------------------
| Register |   Name   |       Notes        |
--------------------------------------------
|    r0    |   $zero  | Always 0           |
|    r1    |   $at    | For assembler      |
|    r2    |   $v0    | Stores results     |
|    r3    |   $v1    | Stores results     |
|    r4    |   $a0    | Stores arguments   |
|    r5    |   $a1    | Stores arguments   |
|    r6    |   $sp    | Stack pointer      |
|    r7    |   $ra    | Return address     |
--------------------------------------------
```


# Instruction Set Architecture Descriptive Info:
## R-Type
### Encoding
```
    4         3        3        3         3
-------------------------------------------------
|   OP   |   $1   |   $2   |   $3    |   FUNCT  |
-------------------------------------------------
0                                              16
```
- OP: opcode - always 0000 for R-Type instructions
- $1: register to write the result of the operation
- $2/$3: registers containing the first/second argument of the operation
- FUNCT: function code for the operation

### Instructions
#### ADD:
- Instruction format: `ADD $1,$2,$3`
- Description: Adds the values in registers $2 and $3 and stores the result in $1
- Math: $1 = $2 + $3
- Note: N/A
- Data Path: r-type.png
- Encoding:
    ```
         4         3        3        3         3
    -------------------------------------------------
    |   0000   |   $1   |   $2   |   $3    |   000  |
    -------------------------------------------------
    0                                              16
    ```

#### SUB:
- Instruction format: `SUB $1,$2,$3`
- Description: Substracts the values in $2 and $3 and stores the result in $1
- Math: $1 = $2 - $3
- Note: N/A
- Data Path: r-type.png
- Encoding:
    ```
         4         3        3        3         3
    -------------------------------------------------
    |   0000   |   $1   |   $2   |   $3    |   001  |
    -------------------------------------------------
    0                                              16
    ```

#### AND:
- Instruction format: `AND $1,$2,$3`
- Description: Bitwise ands the values in $2 and $3 and stores the result in $1
- Math: $1 = $2 && $3
- Note: N/A
- Data Path: r-type.png
- Encoding:
    ```
         4         3        3        3         3
    -------------------------------------------------
    |   0000   |   $1   |   $2   |   $3    |   010  |
    -------------------------------------------------
    0                                              16
    ```

#### OR:
- Instruction format: `OR $1,$2,$3`
- Description: Bitwise ors the values in $2 and $3 and stores the result in $1
- Math: $1 = $2 || $3
- Note: N/A
- Data Path: r-type.png
- Encoding:
    ```
         4         3        3        3         3
    -------------------------------------------------
    |   0000   |   $1   |   $2   |   $3    |   011  |
    -------------------------------------------------
    0                                              16
    ```

#### NOR:
- Instruction format: `NOR $1,$2,$3`
- Description: Bitwise nors the values in $2 and $3 and stores the result in $1
- Math: $1 = !($2 || $3)
- Note: N/A
- Data Path: r-type.png
- Encoding:
    ```
         4         3        3        3         3
    -------------------------------------------------
    |   0000   |   $1   |   $2   |   $3    |   100  |
    -------------------------------------------------
    0                                              16
    ```

#### SLL:
- Instruction format: `SLL $1,$2,$3`
- Description: Left shifts the value in $2 by the value in $3 and stores the result in $1
- Math: $1 = $2 << $3
- Note: Does not rotate
- Data Path: r-type.png
- Encoding:
    ```
         4         3        3        3         3
    -------------------------------------------------
    |   0000   |   $1   |   $2   |    $3   |   101  |
    -------------------------------------------------
    0                                              16
    ```

#### SRL:
- Instruction format: `SRL $1,$2,$3`
- Description: Right shifts the value in $2 by the value in $3 and stores the result in $1
- Math: $1 = $2 >> $3
- Note: Does not rotate, does not preserve the sign bit
- Data Path: r-type.png
- Encoding:
    ```
         4         3        3        3         3
    -------------------------------------------------
    |   0000   |   $1   |   $2   |    $3   |   110  |
    -------------------------------------------------
    0                                              16
    ```

#### SRA:
- Instruction format: `SRA $1,$2,$3`
- Description: Right shifts the value in $2 by the value in $3 and stores the result in $1
- Math: $1 = $2 >> $3
- Note: Does not rotate, preserves the sign bit
- Data Path: r-type.png
- Encoding:
    ```
         4         3        3        3         3
    -------------------------------------------------
    |   0000   |   $1   |   $2   |    $3   |   111  |
    -------------------------------------------------
    0                                              16
    ```

## J-Type
### Encoding
```
     4                       12
--------------------------------------------------
|    OP    |               ADDRESS               |
--------------------------------------------------
0                                              16
```
- OP: opcode of the operation
- ADDRESS: shortened address of the instruction destination

### Instructions
#### JMP:
- Instruction format: `JMP ADDRESS`
- Description: Jumps to the given pseudo-address
- Math: PC = PC[0:3] + (ADDRESS << 1)
- Note: Sets the PC to its highest 3 bits plus the pseudo-address and a trailing 0 (for alignment)
- Data Path: j-type.png
- Encoding:
    ```
         4                       12
    --------------------------------------------------
    |   0001   |               ADDRESS               |
    --------------------------------------------------
    0                                              16
    ```

#### JAL:
- Instruction format: `JAL ADDRESS`
- Description: Jumps to the given pseudo-address and stores the current value of PC + 2 into the $ra register
- Math: $ra = PC + 2, PC = PC[0:3] + (ADDRESS << 1)
- Note: Sets the PC to its highest 3 bits plus the pseudo-address and a trailing 0 (for alignment)
- Data Path: j-type.png
- Encoding:
    ```
         4                       12
    --------------------------------------------------
    |   0010   |               ADDRESS               |
    --------------------------------------------------
    0                                              16
    ```

## I-Type
### Encoding
```
     4         3        3              6
--------------------------------------------------
|    OP    |   $1   |   $2   |     IMMEDIATE     |
--------------------------------------------------
0                                              16
```
- OP: opcode of the instruction
- $1: destination register of the operation
- $2: register containing first argumnent of the operation
- $3: immediate value relating to the operation

### Instructions
#### LW:
- Instruction format: `LW $1,IMMEDIATE($2)`
- Description: Loads a word from memory into $1 starting from the address stored in $2 plus the immediate
- Math: $1 = MEMORY[$2 + IMMEDIATE]
- Note: N/A
- Data Path: load-i-type.png
- Encoding:
    ```
         4         3        3              6
    --------------------------------------------------
    |   0011   |   $1   |   $2   |     IMMEDIATE     |
    --------------------------------------------------
    0                                              16
    ```

#### SW:
- Instruction format: `SW $1,IMMEDIATE($2)`
- Description: Stores a word into memory at $2 plus the immediate with the value in $1
- Math: MEMORY[$2 + IMMEDIATE] = $1
- Note: N/A
- Data Path: store-i-type.png
- Encoding:
    ```
         4         3        3              6
    --------------------------------------------------
    |   0100   |   $1   |   $2   |     IMMEDIATE     |
    --------------------------------------------------
    0                                              16
    ```

#### BEQ:
- Instruction format: `BEQ $1,$2,IMMEDIATE`
- Description: Sets the PC to the value of the PC plus the sign-extended IMMEDIATE with a trailing 0, if $1 is equal to $2
- Math: PC = PC + (IMMEDIATE << 1)
- Note: We shift the IMMEDIATE by 1 for allignemnt
- Data Path: branch-i-type.png
- Encoding:
    ```
         4         3        3              6
    --------------------------------------------------
    |   0101   |   $1   |   $2   |     IMMEDIATE     |
    --------------------------------------------------
    0                                              16
    ```

#### BNE:
- Instruction format: `BNE $1,$2,IMMEDIATE`
- Description: Sets the PC to the value of the PC plus the sign-extended IMMEDIATE with a trailing 0, if $1 is not equal to $2
- Math: PC = PC + (IMMEDIATE << 1)
- Note: We shift the IMMEDIATE by 1 for allignemnt
- Data Path: branch-i-type.png
- Encoding:
    ```
         4         3        3              6
    --------------------------------------------------
    |   0110   |   $1   |   $2   |     IMMEDIATE     |
    --------------------------------------------------
    0                                              16
    ```

#### ADDI:
- Instruction format: `ADDI $1,$2,IMMEDIATE`
- Description: Adds the immediate value to $2 and stores in $1
- Math: $1 = $2 + IMMEDIATE
- Note: N/A
- Data Path: i-type.png
- Encoding:
    ```
         4         3        3              6
    --------------------------------------------------
    |   0111   |   $1   |   $2   |     IMMEDIATE     |
    --------------------------------------------------
    0                                              16
    ```

#### SUBI:
- Instruction format: `SUBI $1,$2,IMMEDIATE`
- Description: Subtracts the immediate value to $2 and stores in $1
- Math: $1 = $2 - IMMEDIATE
- Note: N/A
- Data Path: i-type.png
- Encoding:
    ```
         4         3        3              6
    --------------------------------------------------
    |   1000   |   $1   |   $2   |     IMMEDIATE     |
    --------------------------------------------------
    0                                              16
    ```

#### JMPI:
- Instruction format: `JMPI IMMEDIATE($1)`
- Description: Jumps to the given address in $1 plus IMMEDIATE shifted by 1
- Math: PC = $1 + (IMMEDIATE << 1)
- Note: We shift the IMMEDIATE by 1 for allignemnt
- Encoding:
    ```
         4         3                  9
    --------------------------------------------------
    |   1001   |   $1   |         IMMEDIATE          |
    --------------------------------------------------
    0                                              16
    ```

#### JALI:
- Instruction format: `JALI IMMEDIATE($1)`
- Description: Jumps to the given address in $1 plus IMMEDIATE shifted by 1 and stores the current value of PC + 2 in $ra
- Math: $ra = PC + 2, PC = $1 + (IMMEDIATE << 1)
- Note: We shift the IMMEDIATE by 1 for allignemnt
- Encoding:
    ```
         4         3                  9
    --------------------------------------------------
    |   1001   |   $1   |         IMMEDIATE          |
    --------------------------------------------------
    0                                              16
    ```

#### LI:
- Instruction format: `LI $1, IMMEDIATE`
- Description: Loads the given immediate into $1
- Math: $1 = IMMEDIATE
- Note: N/A
- Encoding:
    ```
         4         3                  9
    --------------------------------------------------
    |   1001   |   $1   |         IMMEDIATE          |
    --------------------------------------------------
    0                                              16
    ```

