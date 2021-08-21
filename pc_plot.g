; setting home position
G28.1 X400 Y400 Z1
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

; move to position 600/400
G0 X600 Y400 Z0

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

; move to position 400/700
G0 X400 Y700 Z0

; set relative positioning
G91
G0 Z1

G02 X420 Y720 I0 J-20