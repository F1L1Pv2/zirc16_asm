main:
    hlt
    limb R1, stack
    lim R2, ((5 + 5) << 2 )
    psh R1, R2
.loop:
    lim R1, .loop
    lsh R1, 1
    brc z, (data.ballz & 0x3F)
data:
    .ballz: dw "Hello World!\n",0
stack:
