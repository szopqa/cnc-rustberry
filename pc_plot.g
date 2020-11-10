; setting home position
G28.1 X400 Y400 Z1
; move to home position
G28
; set relative positioning
G91

; move sequence
G0 X0 Y-100 Z1
G0 X100 Y100 Z1
G0 X-100 Y0 Z1
G0 X100 Y-100 Z1
G0 X-100 Y0 Z1
G0 X50 Y-50 Z1
G0 X50 Y50 Z1
G0 X0 Y100 Z1

; set absolute positioning
G90

; move to position 800/800
G0 X600 Y400 Z0

; set relative positioning
G91

; move sequence
G0 X0 Y-100 Z1
G0 X100 Y100 Z1
G0 X-100 Y0 Z1
G0 X100 Y-100 Z1
G0 X-100 Y0 Z1
G0 X50 Y-50 Z1
G0 X50 Y50 Z1
G0 X0 Y100 Z1