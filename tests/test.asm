        BS SFR_PORT_DIR, SB_PORT_DIR1   ; set IO1 to output
LOOP:   BS SFR_PORT_IO, SB_PORT_OUT1    ; set IO1 to high
        NOP
        NOP
        BC SFR_PORT_IO, SB_PORT_OUT1    ; set IO1 to low
        JMP LOOP                        ; jump takes 2 cycles
