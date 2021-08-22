; setting home position
G28.1 X100 Y600 Z1
; move to home position
G28
; set relative positioning
G91

; move sequence
G0 
X0 Y-100 Z1
X100 Y100 Z1
X-100 Y0 Z1
X100 Y-100 Z1
X-100 Y0 Z1
X50 Y-50 Z1
X50 Y50 Z1
X0 Y100 Z1

; set absolute positioning
G90

; move to position 300/600
G0 X300 Y600 Z0

; set relative positioning
G91

; move sequence
G0 
X0 Y-100 Z1
X100 Y100 Z1
X-100 Y0 Z1
X100 Y-100 Z1
X-100 Y0 Z1
X50 Y-50 Z1
X50 Y50 Z1
X0 Y100 Z1

; set absolute positioning
G90

; move to position 200/300
G0 X200 Y300 Z1

; draw circle
G02
X300 Y400 I-100 J-100