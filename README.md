instructions are 2 bits long

| Opcode | Description |
|--------|-------------|
| 000X | Jump to memory adress X |
| 10XY | If register X is equal to register Y, skip the next instruction |
| 20XY | If register X is not equal to register Y, skip the next instruction |
| 30XY | If register X is less than register Y, skip the next instruction |
| 40XY | If register X is greater than register Y, skip the next instruction |
| 5XNN | Add NN to register X |
| 60XY | Set register X to register Y subtracted from register X |
| 70XY | Set register X to register Y |
| 8XNN | Set register X to the value of Y |

