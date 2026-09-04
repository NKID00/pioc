LOOP:   BS SFR_PORT_IO, SB_PORT_OUT1    ; set IO1 to high
        NOP
        BC SFR_PORT_IO, SB_PORT_OUT1    ; set IO1 to low
        JMP LOOP
