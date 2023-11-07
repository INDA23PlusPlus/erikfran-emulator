Simplified ISA based on CHIP-8
* instructions are 16 bits long
* registers are V0-VF
* all registers are 8 bits (0x00-0xFF) (0-255)
* VF is reserved for flags for some math ops (carry, borrow, etc)
* memory is addressed by 8 bits (0x00-0xFF) (0-255). Every address holds 1 byte (8 bits)
* address 0xFF are reserved for output. If a program writes to this address, it will be printed to the screen
* AA is a 8bit constant
* PC is the program counter (8 bits)


| Opcode | Type | Pseudo Code | Assembly | Description |
|-|-|-|-|-|
| 00NN | Flow | goto(NN) | jump NN | Jump to memory address NN. Same as setting PC to NN |
| 10XY | Cond | if VX == VY | ifeq VX VY | If VX is equal to VY, skip the next instruction |
| 20XY | Cond | if VX != VY | ifneq VX VY | If register X is not equal to register Y, skip the next instruction |
| 30XY | Cond | if VX < VY | ifle VX VY | If register X is less than register Y, skip the next instruction |
| 4XNN | Const | VX = 0xAA | setva VX AA | Set VX to AA |
| 50XY | Reg | VX = VY | setvv VX VY | Set VX to VY |
| 510X | Reg | VX = PC | setvpc VX | Set VX to PC |
| 6XNN | Reg | VX = NN | setvn | Set VX to NN |
| 7XAA | Reg | VX = AA | setva VX AA | Set VX to AA |
| 8XNN | PC | PC = VX | setpcv VX | Set PC to VX |
| 9NNX | Mem | NN = VX | setnv NN VX | Set memory address NN to VX |
| A0XY | Math | VX = VX + VY | add VX VY | Add VX to VY and store the result in VX. If the result is greater than 255 (0xFF), set the carry flag VF to 1, otherwise set VF to 0 |
| A1XY | Math | VX = VX - VY | sub VX VY | Subtract VY from VX and store the result in VX. If VY is greater than VX, set the borrow flag VF to 1, otherwise set VF to 0 |
| B0XY | BitOp | VX = VX & VY | and VX VY | Set VX to VX AND VY |
| B1XY | BitOp | VX = VX \ VY | or VX VY | Set VX to VX OR VY |
| B2XY | BitOp | VX = VX ^ VY | xor VX VY | Set VX to VX XOR VY |
| B3XY | BitOp | VX = ~VX | not VX | Set VX to NOT VX |

