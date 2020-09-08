G0 X12 Y1
G1 X7 Y18 Z33

; not supported yet G17 
; not supported yet G20 
; not supported yet G90 
; not supported yet G94 
; not supported yet G54
G0 Z0.25
X-0.5 Y0.
Z0.1
G01 Z0. F5.
; not supported yet G02 X0. Y0.5 I0.5 J0. F2.5
; not supported yet X0.5 Y0. I0. J-0.5
; not supported yet X0. Y-0.5 I-0.5 J0.
; not supported yet X-0.5 Y0. I0. J0.5
G01 Z0.1 F5.
G00 X0. Y0. Z0.25
G28
