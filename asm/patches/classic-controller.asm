.open "main.dol"

; comment out KPADEnableMpls call -- this will cause
; extension data to be unconditionally available
.org 0x80055378
nop
nop
nop

; dPad::ex_c::isMissingMpls()
.org 0x80058BC0
; we finished MPLS calibration :)
lwz r3, sInstance__13dPadManager_c@sda21(r13)
li r4, 1
stb r4, 0x21(r3)
; we are not missing Motion Plus :)
li r3, 0
blr

; dPad::ex_c::isMissingNunchuk()
; we are not missing the Nunchuk :)
.org 0x80058C20
li r3, 0
blr

; EGG::CoreController::beginFrame(PADStatus*)
; redirect KPADReadEx to our wrapper
.org 0x8049968C
bl kpad_read_ex_wrapper


; cursor looking - TODO: Link cannot turn enough e.g. between Lanayru Caves and Ancient Harbor
.org 0x80207a34
bl get_aiming_stick_dir


; item select - TODO: Not sure when this is hit, this is part of MsgWindowSelectBtn
; .org 0x8011c984
; bl get_item_select_stick_dir
; .org 0x8011c9ac
; bl get_item_select_stick_dir
; .org 0x8011ccd0
; bl get_item_select_stick_dir
; .org 0x8011cdcc
; bl get_item_select_stick_dir
; .org 0x8011cd10
; bl get_item_select_stick_dir


; item select - actual player stuff
.org 0x801e3f0c
bl calc_item_select_length_angle


; beetle flying
.org 0x80263bcc
bl get_beetle_flying_zrot

.org 0x80263c40
bl get_beetle_flying_yrot

.close