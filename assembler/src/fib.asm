@N =  12

setrc 0x1 0x01      /* V1 = 1 */
setrc 0x5 0x0C      /* V5 = @N */

@LOOP
    ifle  2    5    /* if V2 is less than V5 skips next line so that program does not end */
    jump  0x0D      /* jump to @END. Will end th program */

    setmr 1    0xFF /* print(V1) */
    setrr 3    1    /* V3 = V1 */
    add   3    0    /* V3 = V3 + V1 */
    setrr 0    1    /* V0 = V1 */
    setrr 1    3    /* V1 = V3 */
    setrc 4    0x01 /* V4 = 0x01 */
    add   2    4    /* V2 = V2 + V4 */
    jump  @LOOP     /* jump back to start of loop */

@END