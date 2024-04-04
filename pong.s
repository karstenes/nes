.segment "HEADER"       ; Setting up the header, needed for emulators to understand what to do with the file, not needed for actual cartridges
	.byte	"NES", $1A	; iNES header identifier
	.byte	2		; 2x 16KB PRG code
	.byte	1		; 1x  8KB CHR data
	.byte	$01, $00	; mapper 0, vertical mirroring

.segment "ZEROPAGE"
  ; Define constants ;;;;;;;;;;;;;;;;;;;;;;;

  ;; PPU Registers`
  PPUCTRL   = $2000
  PPUMASK   = $2001
  PPUSTATUS = $2002
  OAMADDR   = $2003
  OAMDATA   = $2004
  PPUSCROLL = $2005
  PPUADDR   = $2006
  PPUDATA   = $2007
  OAMDMA    = $4014
  ;; Other Registers
  CONTROL1  = $4016
  CONTROL2  = $4017

  ;; Sprite Attributes
  ;; Botwise OR together to get value
  ;; Ex: (FLIP_H | SPR_PAL_0)
  SPR_NORMAL  = %00000000
  SPR_PAL_0   = %00000000
  SPR_PAL_1   = %00000001
  SPR_PAL_2   = %00000010
  SPR_PAL_3   = %00000011
  SPR_FLIP_H  = %01000000
  SPR_FLIP_V  = %10000000

  ;; Controller Buttons
  ; A B SEL START UP DOWN LEFT RIGHT
  BUTTON_A      = %10000000
  BUTTON_B      = %01000000
  BUTTON_SELECT = %00100000
  BUTTON_START  = %00010000
  BUTTON_UP     = %00001000
  BUTTON_DOWN   = %00000100
  BUTTON_LEFT   = %00000010
  BUTTON_RIGHT  = %00000001

  ;; State Values
  STATE_TITLE   = $00
  STATE_GAME    = $01
  STATE_END     = $02

  ;; Initial Values
  PLAYER1_START_X = $08
  PLAYER1_START_Y = $68
  PLAYER2_START_X = $F0
  PLAYER2_START_Y = $68
  BALL_START_X    = $7C
  BALL_START_Y    = $74

  ;; Misc Constants
  PADDLE_SPEED    = $02
  BALL_SPEED      = $01
  BALL_STATE_STOP = %00000000
  BALL_STATE_PLAY = %10000000
  RESET_BALL      = %01111111
  RESET_BALL_DIR  = %11111100
  BALL_DIR_RIGHT  = %00000001
  BALL_DIR_LEFT   = %00000000
  BALL_DIR_DOWN   = %00000010
  BALL_DIR_UP     = %00000000
  MESSAGE_UPDATE  = %00100000
  CLEAR_MESSAGE   = %11011111
  SCORE_UPDATE    = %01000000
  CLEAR_SCORE     = %10111111
  MAX_SCORE       = %00001110
  ONE_PLAYER_GAME = $00
  TWO_PLAYER_GAME = $01
  MAX_UP          = $08
  MAX_DOWN        = $C6
  BALL_MAX_UP     = $08
  BALL_MAX_DOWN   = $E6

  AI_SPEED        = $01
  AI_UP           = $01
  AI_DOWN         = $00
  AI_STAND        = $FF

  ; Timing variables
  frame_count:      .res 1
  seconds:          .res 1

  ; Pointers
  pointer_lo:       .res 1  ; pointer variables declared in RAM
  pointer_hi:       .res 1  ; low byte first, high byte immediately after
  RLE_tag:          .res 1
  RLE_byte:         .res 1

  ; Define Global Variables
  control1_buttons:       .res 1
  control2_buttons:       .res 1
  control_1_old_buttons:  .res 1
  ; control_2_old_buttons:  .res 1
  disable_input:          .res 1
  disable_input_seconds:  .res 1

  ; State variables
  game_state:       .res 1
  title_loaded:     .res 1
  bg_cleared:       .res 1   
  game_over_loaded: .res 1
  num_players:      .res 1

  ; Game variables

  ;; GAME FLAGS
  ;;        1═      1 - ball going right | 0 - ball going left
  ;;       2══      1 - ball going down | 0 - ball going up
  ;;      3═══      
  ;;     4════      
  ;;    5═════      
  ;;   6══════      1 - message needs update | 0 - no message update
  ;;  7═══════      1 - score needs update | 0 - score needs no update
  ;; 8════════      1 - ball in play | 0 - ball stopped
  game_flags:       .res 1

  score_p1:         .res 1
  score_p2:         .res 1

  player_1_x:       .res 1
  player_1_y:       .res 1
  player_2_x:       .res 1
  player_2_y:       .res 1
  ball_x:           .res 1
  ball_y:           .res 1

  ai_direction:     .res 1


.segment "STARTUP"
.segment "CODE"

.proc decodeRLE
	stx pointer_lo
	sty pointer_hi
	ldy #0
	jsr doRLEbyte
	sta RLE_tag
L1:
	jsr doRLEbyte
	cmp RLE_tag
	beq L2
	sta PPUDATA
	sta RLE_byte
	bne L1
L2:
	jsr doRLEbyte
	cmp #0
	beq L4
	tax
	lda RLE_byte
L3:
	sta PPUDATA
	dex
	bne L3
	beq L1
L4:
	rts
.endproc

.proc doRLEbyte
	lda (pointer_lo),y
	inc pointer_lo
	bne L1
	inc pointer_hi
L1:
	rts
.endproc

vblank_wait:      ; wait for vblank to make sure PPU is ready
:
  BIT PPUSTATUS   ; returns bit 7 of ppustatus reg, which holds the vblank status with 0 being no vblank, 1 being vblank
  BPL :-          ; branch if bit is positive
  RTS

; @
; @ dma_transfer
; @ Does DMA transfer of the sprites from memory
; @ block $0200
; @
dma_transfer:
  LDA #$00 
  STA OAMADDR    ; set low byte of RAM (00), start transfer
  ; do DMA transfer from the $0200 block of RAM to the PPU via OAM DMA
  LDA #$02
  STA OAMDMA     ; set high byte of RAM (02), start transfer
  RTS
; end dma_transfer

; @
; @ disable_rendering
; @ Disables rendering in the PPU
; @
disable_rendering:
  LDA #$00
  STA PPUMASK
  RTS
; end disable_rendering

; @
; @ setup_ppu
; @ Gets PPU ready to draw
; @
setup_ppu:
  LDA #%10010000          ; enable NMI, sprites from Pattern Table 0, background from pattern table 1
  STA PPUCTRL             ; the left most bit of $2000 sets whether NMI is enabled or not
  LDA #%00011110	        ; enable sprites and background, no clipping on left
  STA PPUMASK
  RTS
; end setup_ppu

; @
; @ setup_scrolling
; @ Sets no scrolling mode 
; @
setup_scrolling:
  LDA #$00          ; tell the PPU there is no background
  STA PPUSCROLL     ; scrolling
  STA PPUSCROLL
  RTS
; end setup_scrolling

; @
; @ ready_ppu_background
; @ Gets the PPU ready to draw the background
; @
ready_ppu_background:
  LDA PPUSTATUS
  LDA #$20
  STA PPUADDR
  LDA #$00
  STA PPUADDR
  RTS
; end ready_ppu_background

; @
; @ ready_ppu_attributes
; @ Gets the PPU ready to draw the background attributes
; @
ready_ppu_attributes:
  LDA PPUSTATUS           ; read PPU Status to reset high/low latch
  LDA #$23                ; high byte of $23C0
  STA PPUADDR             ; write to PPU Addresser
  LDA #$c0                ; low byte of $23C0
  STA PPUADDR
  RTS
; end ready_ppu_attributes

; @
; @ update_timer
; @ Updates the frame counter and second counter
; @
update_timer:
  INC frame_count
  LDA frame_count
  CMP #$3C                ; compare to 60 ( $3C )
  BCC @finish
  INC seconds
  LDA #$00
  STA frame_count
@finish:
  RTS
; end update_timer

; @
; @ set_initial_positions
; @ Sets the sprites to their intial positions
; @
set_initial_positions:
  LDA #PLAYER1_START_X
  STA player_1_x
  LDA #PLAYER1_START_Y
  STA player_1_y
  LDA #PLAYER2_START_X
  STA player_2_x
  LDA #PLAYER2_START_Y
  STA player_2_y
  LDA #BALL_START_X
  STA ball_x
  LDA #BALL_START_Y
  STA ball_y
  RTS
; end set_initial_positions

; @
; @ set_state_title
; @ Sets the game state to the title state
; @ also sets that title screen is not loaded
; @
set_state_title:
  LDA #STATE_TITLE
  STA game_state
  STA title_loaded
  RTS
; end set_state_title

; @
; @ set_state_game
; @ Sets the game state to the game state
; @ also sets that bg needs to be cleared,
; @ resets scores to 0, and sets flags
; @ to update the message and scores on screen
; @
set_state_game:
  LDA #STATE_GAME
  STA game_state
  LDA #$00
  STA bg_cleared
  STA score_p1
  STA score_p2
  ORA #(MESSAGE_UPDATE | SCORE_UPDATE)
  STA game_flags
  RTS
; end set_state_game

; @
; @ set_state_end
; @ Sets the game state to the game over state
; @ also sets that title screen is not loaded
; @
set_state_end:
  LDA #STATE_END
  STA game_state
  LDA #$00
  STA game_over_loaded
  RTS
; @ end set_state_end

; @
; @ reset_sprites
; @ Reloads the sprites from ROM
; @
reset_sprites:
  LDX #$00                ; x = 0
@loop:
  LDA sprite_data, x      ; load byte from ROM address ( sprite_data + x as offset )
  STA $0200, x            ; store into RAM address ( $0200 + x as offset )
  INX
  CPX #$28                ; x == $20? - each sprite holds 4 bytes of data - Ycoord, tile, attribs, & Xcoord - 8 sprites, so 8*4 = 32 or $20
  BNE @loop               ; No, jump to load_sprites_loop; yes, fall through
  RTS
; end reset_sprites

sound_wall_hit:
  LDA #%00000001
  STA $4015
  LDA #%01010100
  STA $4000
  LDA #$C9
  STA $4002
  LDA #%00010001
  STA $4003
  RTS

sound_point_scored:
  LDA #%00000001
  STA $4015
  LDA #%01010100
  STA $4000
  LDA #$E4
  STA $4002
  LDA #%00110000
  STA $4003
  RTS

RESET:
  SEI             ; disable IRQs
  CLD             ; disable decimal mode
  LDX #$40
  STX $4017       ; disable APU frame counter IRQ - disable sound
  LDX #$ff
  TXS             ; setup stack starting at FF as it decrements instead if increments
  INX             ; overflow X reg to $00
  STX PPUCTRL     ; disable NMI - PPUCTRL reg
  STX PPUMASK     ; disable rendering - PPUMASK reg
  STX $4010       ; disable DMC IRQs

  JSR vblank_wait

  LDA #$00        ; can also do TXA as x is $#00
clear_memory:
	STA	$0000, x
	STA	$0100, x
	STA	$0300, x
	STA	$0400, x
	STA	$0500, x
	STA	$0600, x
	STA	$0700, x
  LDA #$fe
  STA $0200, x      ; move all sprites off screen
	INX
  BNE clear_memory  ; branches if not zero

  JSR vblank_wait

clear_nametables:
  LDA PPUSTATUS     ; read PPU status to reset the high/low latch
  LDA #$20          ; write the high byte of $2000
  STA PPUADDR
  LDA #$00          ; write the low byte of $2000
  STA PPUADDR
  LDX #$08          ; prepare to fill 8 pages ($800 bytes)
  LDY #$00          ; x/y is 16 bit counter, high byte in x
  LDA #$24          ; fill with tile $24 (a sky tile)
@loop:
  STA PPUDATA
  DEY
  BNE @loop
  DEX
  BNE @loop

;;;;;;;;;;; MAIN CODE BEGINS ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

  ; The color palettes are located in VRAM at $3f00 - $3f1f
  ; Load from palette in ROM.
load_palettes:
  LDA PPUSTATUS   ; read PPU status to reset the high/low latch
  LDA #$3f
  STA PPUADDR     ; write high byte of $3F00 address
  LDA #$00
  STA PPUADDR     ; write low byte of $3F00 address
                  ; the $3F00 address is the memory location for the background palette, going to $3F0F (16 bytes)
                  ; the sprite palette is at $3F10, ending at $3F1F, which is 32 bytes > $3F00, so want to loop 32 times
  LDX #$00
@loop:
  LDA palette_data, x     ; load byte from ROM address ( palette_data + x as offset )
  STA PPUDATA             ; write to PPU
  INX
  CPX #$20                ; x == $20 (32 decimal) ?? - loop 32 times to write address from $3F00 to $3F1F
  BNE @loop               ; No, jump to load_palettes_loop; yes, fall through

  JSR vblank_wait

load_sprites:
  JSR reset_sprites

load_nametable:
  LDA PPUSTATUS           ; read PPU Status to reset high/low latch
  LDA #$20
  STA PPUADDR             ; write high byte of $2000 address
  LDA #$00
  STA PPUADDR             ; write low byte
  LDX #$00
@loop:
  LDA background, x   
  STA PPUDATA             ; write to PPU
  INX
  CPX #$80                ; compare X to $80 (dec 128) as copying 128 bytes
  BNE @loop

load_attribute:
  JSR ready_ppu_attributes
@loop:
  LDA attribute_data, x
  STA PPUDATA
  INX
  CPX #$08                ; copying 8 bytes of data
  BNE @loop

set_initial_state:
  LDA #ONE_PLAYER_GAME
  STA num_players
  JSR set_initial_positions
  JSR set_state_title

@setup_ppu:
  CLI                     ; clear interrupts so NMI can be called
  JSR setup_ppu

  ; Main loop. Everything happens in VBLANK, so this just spins.
forever:
  JMP forever     ; an infinite loop when init code is run

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

VBLANK:
  ; save registers
  PHA             
  TXA
  PHA
  TYA
  PHA

  JSR dma_transfer
  LDA game_state

state_check:
  CMP #STATE_TITLE
  BEQ do_title

  CMP #STATE_GAME
  BEQ do_game

  CMP #STATE_END
  BEQ do_end
  JMP after_state_check

do_title:
  JSR load_title
  JSR draw_cursor
  JMP after_state_check

do_game:
  JSR clear_background
  JSR start_message
  JSR update_score
  JSR move_ball
  JMP after_state_check

do_end:
  JSR reset_sprites
  JSR game_over_screen
  JMP after_state_check

after_state_check:

  ;; PPU clean up section
  JSR setup_scrolling
  JSR setup_ppu
  
  JSR read_controller_1
  JSR read_controller_2

  JSR update_timer

  LDA disable_input
  CMP #$01
  BNE @input_not_disabled
  LDA disable_input_seconds
  CLC
  ADC #$01
  CMP seconds
  BCC @enable_input
  JMP game_update_sprites

@enable_input:
  LDA #$00
  STA disable_input

@input_not_disabled:
;; button order: A, B, Select, Start, Up, Down, Left, Right ;;;;
read_a:
  LDA control1_buttons
  AND #BUTTON_A  
  BEQ end_read_a
  ;; code for when pressed
  LDA game_flags
  AND #BALL_STATE_PLAY
  BNE end_read_a

  ; we need to update ball state and update message now
  LDA game_flags
  ORA #(MESSAGE_UPDATE|BALL_STATE_PLAY)  
  STA game_flags   
  JSR launch_ball   

end_read_a:

read_b:
  LDA control1_buttons
  AND #BUTTON_B  
  BEQ end_read_b
  ;; code for when pressed
  DEC ball_x
end_read_b:

read_select:
  LDA control1_buttons
  AND #BUTTON_SELECT
  BEQ end_read_select
  ;; code for when pressed
  LDA game_state
  CMP #STATE_TITLE
  BNE end_read_select

  LDA control_1_old_buttons
  AND #BUTTON_SELECT
  BNE end_read_select
  JSR toggle_num_players
  
end_read_select:

read_start:
  LDA control1_buttons   
  AND #BUTTON_START
  BEQ end_read_start
  ;; code for when pressed
  LDA game_state
  CMP #STATE_TITLE
  BNE @not_title

  JSR set_initial_positions
  JSR set_state_game

@not_title:
  CMP #STATE_END
  BNE end_read_start
  LDA #$01
  STA disable_input
  LDA seconds
  STA disable_input_seconds
  JSR set_state_title

end_read_start:

read_up:
  LDA game_state
  CMP #STATE_GAME
  BNE end_read_down

  LDA control1_buttons
  AND #BUTTON_UP
  BEQ @check_p2_up
  ;; code for when pressed
  LDA player_1_y
  CMP #MAX_UP
  BEQ @check_p2_up
  SEC
  SBC #PADDLE_SPEED
  STA player_1_y

@check_p2_up:
  LDA num_players         ; Check if we need to read p2 input
  CMP #TWO_PLAYER_GAME
  BNE end_read_up

  LDA control2_buttons
  AND #BUTTON_UP
  BEQ end_read_up
  ;; code for when pressed
  LDA player_2_y
  CMP #MAX_UP
  BEQ end_read_up
  SEC
  SBC #PADDLE_SPEED
  STA player_2_y

end_read_up:

read_down:
  LDA game_state
  CMP #STATE_GAME
  BNE end_read_down

  LDA control1_buttons
  AND #BUTTON_DOWN
  BEQ @check_p2_down
  ;; code for when pressed
  LDA player_1_y
  CMP #MAX_DOWN
  BEQ @check_p2_down
  CLC
  ADC #PADDLE_SPEED
  STA player_1_y

@check_p2_down:
  LDA num_players         ; Check if we need to read p2 input
  CMP #TWO_PLAYER_GAME
  BNE end_read_down

  LDA control2_buttons
  AND #BUTTON_DOWN
  BEQ end_read_down
  ;; code for when pressed
  LDA player_2_y
  CMP #MAX_DOWN
  BEQ end_read_down
  CLC
  ADC #PADDLE_SPEED
  STA player_2_y

end_read_down:

read_left:
  LDA control1_buttons
  AND #BUTTON_LEFT
  BEQ end_read_left
  ;; code for when pressed

end_read_left:

read_right:
  LDA control1_buttons
  AND #BUTTON_RIGHT
  BEQ end_read_right
  ;; code for when pressed
 
end_read_right:

ai_moves:
  LDA num_players           ; check if one player game
  CMP #ONE_PLAYER_GAME
  BNE game_update_sprites   ; if not, skip AI stuff

  LDA game_state
  CMP #STATE_GAME
  BNE game_update_sprites

  LDA game_flags
  AND #BALL_STATE_PLAY
  BEQ game_update_sprites

  JSR ai_choose_direction
  JSR do_ai_move

game_update_sprites:
  LDA game_state
  CMP #STATE_GAME
  BNE VBLANK_DONE
  JSR update_sprites

VBLANK_DONE:
  ;restore registers
  PLA
  TAY
  PLA
  TAX
  PLA

  RTI
  JMP forever

toggle_num_players:
  LDA num_players
  CMP #ONE_PLAYER_GAME
  BEQ @set_two_player

  LDA #ONE_PLAYER_GAME
  STA num_players
  RTS
@set_two_player:
  LDA #TWO_PLAYER_GAME
  STA num_players
  RTS

draw_cursor:
  LDA #$56    ; set x coord
  STA $0227

  LDA num_players
  CMP #ONE_PLAYER_GAME
  BEQ @one_player

  ; TWO PLAYERS
  LDA #$C7
  STA $0224
  RTS
@one_player:
  LDA #$B7
  STA $0224
  RTS

;
; @ launch_ball
; @ subroutine to launch the ball
;
launch_ball:
  LDA game_flags
  AND #RESET_BALL_DIR
  STA game_flags
  
  LDA frame_count
  AND #%00000011
  ORA game_flags
  STA game_flags
  RTS
; end launch_ball

; @
; @ score_point
; @ Called whenever a point is scored. 
; @ Adds the score and resets the field
; @ Also determines if we're in game over
; @
score_point:
  JSR sound_point_scored
  
  LDA ball_x            ; if ball x is less than mid-screen, 
  CMP #$80              ; it's hitting the left side
  BCC @score_p2_point   ; that's player 2's point; else, fall through

  LDA score_p1

  TAX 
  INX
  STX score_p1

  LDA score_p1
  CMP #MAX_SCORE
  BEQ @game_over

  JMP end_score_point

@score_p2_point:

  LDA score_p2

  TAX 
  INX
  STX score_p2

  LDA score_p2
  CMP #MAX_SCORE
  BEQ @game_over

  JMP end_score_point

@game_over:
  JSR set_state_end
  RTS

end_score_point:
  JSR set_update_score_flag
  JSR set_update_message_flag

  LDA game_flags
  AND #RESET_BALL
  STA game_flags

  JSR set_initial_positions

  RTS
; end score_point

; @
; @ toggle_ball_x
; @ reverse the ball x direction
; @
toggle_ball_x:
  LDA game_flags
  EOR #BALL_DIR_RIGHT
  STA game_flags
  JSR sound_wall_hit
  RTS

; @
; @ toggle_ball_y
; @ reverse the ball y direction
; @
toggle_ball_y:
  LDA game_flags
  EOR #BALL_DIR_DOWN
  STA game_flags
  JSR sound_wall_hit
  RTS

; @
; @ move_ball_x
; @ handles moving the ball left and right
; @ also determines whether we need to score a point
; @
move_ball_x:
  LDA game_flags
  AND #BALL_DIR_RIGHT
  BEQ @move_ball_right  ; move ball right, otherwise fall through
@move_ball_left:
  ; check for collision with paddle
  LDA ball_x
  SEC
  SBC #$08
  CMP player_1_x
  BNE @no_collide_left  ; jump if there is no collision, otherwise fall through

  LDA ball_y            ; check if ball y is greater than player y
  CLC
  ADC #$04
  CMP player_1_y
  BCC @score_point      ; if it's not hitting the paddle, then it's a point

  LDA player_1_y        ; check if ball y is less than player bottom
  CLC
  ADC #$22
  CMP ball_y
  BCC @score_point      ; if it's not hitting the paddle, then it's a point

  JSR toggle_ball_x     ; paddle hit, flip the ball direction
  RTS

  ; no collision, so move ball
@no_collide_left:
  LDA ball_x
  SEC
  SBC #BALL_SPEED
  STA ball_x
  RTS

@move_ball_right:
  ;check for collision with paddle
  LDA ball_x
  CLC
  ADC #$08
  CMP player_2_x
  BNE @no_collide_right

  LDA ball_y            ; check if ball y is greater than player y
  CLC
  ADC #$04
  CMP player_2_y
  BCC @score_point      ; if it's not hitting the paddle, then it's a point

  LDA player_2_y        ; check if ball y is less than player bottom
  CLC
  ADC #$22
  CMP ball_y
  BCC @score_point      ; if it's not hitting the paddle, then it's a point

  JSR toggle_ball_x
  RTS

@no_collide_right:
  LDA ball_x
  CLC
  ADC #BALL_SPEED
  STA ball_x
  RTS

@score_point:
  JSR score_point
  RTS
; end move_ball_x

; @
; @ move_ball_y
; @ handles moving the ball left and right
; @
move_ball_y:
  LDA game_flags
  AND #BALL_DIR_DOWN
  BEQ @move_ball_down
@move_ball_up:
  ;check for collision
  LDA ball_y
  CMP #BALL_MAX_UP
  BNE @no_collide_up

  JSR toggle_ball_y
  JMP end_move_ball_y

@no_collide_up:
  LDA ball_y
  SEC
  SBC #BALL_SPEED
  STA ball_y
  JMP end_move_ball_y

@move_ball_down:
  ; check for collision
  LDA ball_y
  CLC
  ADC #$08
  CMP #BALL_MAX_DOWN                ; bottom of screen
  BNE @no_collide_down

  JSR toggle_ball_y
  JMP end_move_ball_y

@no_collide_down:
  LDA ball_y
  CLC
  ADC #BALL_SPEED
  STA ball_y
end_move_ball_y:
  RTS

; @
; @ move_ball
; @ handles moving the ball
; @ calls the x and y subroutines
; @
move_ball:
  LDA game_flags
  AND #BALL_STATE_PLAY
  BEQ end_move_ball

  JSR move_ball_x

  LDA game_flags
  AND #BALL_STATE_PLAY
  BEQ end_move_ball

  JSR move_ball_y

end_move_ball:
  RTS
; end move_ball

; @
; @ ai_choose_direction
; @ AI chooses which way to move
; @
ai_choose_direction:
  LDA frame_count
  AND #%00011110
  BNE @end_ai_choose    ; only choose direction every 30 frames

  LDA player_2_y
  CMP ball_y
  BCC @choose_down
@choose_up:
  LDA #AI_UP
  STA ai_direction
  RTS
@choose_down:
  LDA #AI_DOWN
  STA ai_direction
  RTS
@end_ai_choose:
  RTS
; end ai_choose_direction

; @
; @ do_ai_move
; @ Actually do the AI's move
; @
do_ai_move:
  LDA ai_direction
  CMP #AI_DOWN
  BEQ @ai_move_down
@ai_move_up:
  LDA player_2_y
  CMP #MAX_UP
  BEQ @end_ai_move
  LDA player_2_y
  SEC
  SBC #AI_SPEED
  STA player_2_y
  RTS
@ai_move_down:
  LDA player_2_y
  CMP #MAX_DOWN
  BEQ @end_ai_move
  LDA player_2_y
  CLC
  ADC #AI_SPEED
  STA player_2_y
@end_ai_move:
  RTS

; @
; @ update_sprites
; @ updates the sprites in memory
; @ in the $0200 block of mem
; @
update_sprites:
@update_x:
  LDA player_1_x
  STA $0203
  STA $0207
  STA $020B
  STA $020F
  LDA player_2_x
  STA $0213
  STA $0217
  STA $021B
  STA $021F
@update_y:
  LDA player_1_y
  STA $0200
  CLC
  ADC #$08
  STA $0204
  CLC
  ADC #$08
  STA $0208
  CLC
  ADC #$08
  STA $020C
  LDA player_2_y
  STA $0210
  CLC
  ADC #$08
  STA $0214
  CLC
  ADC #$08
  STA $0218
  CLC
  ADC #$08
  STA $021C
@update_ball:
  LDA ball_x
  STA $0223
  LDA ball_y
  STA $0220
@update_cursor:
  LDA #$00
  STA $0227
  LDA #$FF
  STA $0224

  RTS
; end update_sprites

; @
; @ clear_background
; @ writes $24 to all background tiles
; @ to the PPU
; @
clear_background:
  LDA bg_cleared
  CMP #$01
  BNE @start
  RTS
@start:
  INC bg_cleared

  JSR disable_rendering
  JSR ready_ppu_background

  LDX #$00
  LDY #$00
@outer_loop:
@inner_loop:
  LDA #$00
  STA PPUDATA

  INY
  CPY #$00
  BNE @inner_loop

  INX 
  CPX #$04
  BNE @outer_loop

@load_attributes:
  JSR ready_ppu_attributes
  LDX #$00
@loop:
  LDA title_attributes, x
  STA PPUDATA
  INX
  CPX #$40                ; copying 64 ($40) bytes of data
  BNE @loop

  RTS
; end clear_background

; @
; @ set_update_score_flag
; @ sets the flag to update the score
; @
set_update_score_flag:
  LDA game_flags
  ORA #SCORE_UPDATE
  STA game_flags
  RTS
; end set_update_score_flag

; @
; @ clear_update_score_flag
; @ clears the flag to update the score
; @
clear_update_score_flag:
  LDA game_flags
  AND #CLEAR_SCORE
  STA game_flags
  RTS
; end clear_update_score_flag


; @
; @ update_p1_score
; @ Updates the scoreboards for player one
; @
update_p1_score:
  ; Set the PPU Address to the position for the scoreboard
  LDA PPUSTATUS
  LDA #$23
  STA PPUADDR
  LDA #$83
  STA PPUADDR

  ;; Draw Player One Scoreboard
  LDA #$F4       ; P
  STA PPUDATA
  LDA #$DC       ; 1
  STA PPUDATA
  LDA #$00       ; _
  STA PPUDATA

  ;; Draw Player One Score Numbers
  LDA score_p1
  CMP #$00
  BEQ @p1_zero_score

  CMP #$0A
  BCC @p1_single_digit

  CMP #$0A
  BEQ @p1_ten

@p1_double_digit:
  LDX #$DC
  STX PPUDATA
  SEC
  SBC #$0A
  CLC
  ADC #$DB
  STA PPUDATA
  RTS

@p1_ten:
  LDX #$DC
  STX PPUDATA
  LDA #$DB
  STA PPUDATA
  RTS

@p1_single_digit:
  LDX #$DB
  STX PPUDATA
  CLC
  ADC #$DB
  STA PPUDATA
  RTS

@p1_zero_score:
  LDA #$DB       ; 0
  STA PPUDATA   
  LDA #$DB       ; 0
  STA PPUDATA
  RTS
; end update_p1_score

; @
; @ update_p2_score
; @ Updates the scoreboards for player two
; @
update_p2_score:
  ; Set the PPU Address to the position for the scoreboard
  LDA PPUSTATUS
  LDA #$23
  STA PPUADDR
  LDA #$98
  STA PPUADDR

  ;; Draw Player Two Scoreboard
  LDA #$F4        ; P
  STA PPUDATA
  LDA #$DD        ; 2
  STA PPUDATA
  LDA #$00        ; _
  STA PPUDATA

  ;; Draw Player Two Score Numbers
  LDA score_p2
  CMP #$00
  BEQ @p2_zero_score

  CMP #$0A
  BCC @p2_single_digit

  CMP #$0A
  BEQ @p2_ten

@p2_double_digit:
  LDX #$DC
  STX PPUDATA
  SEC
  SBC #$0A
  CLC
  ADC #$DB
  STA PPUDATA
  RTS

@p2_ten:
  LDX #$DC
  STX PPUDATA
  LDA #$DB
  STA PPUDATA
  RTS

@p2_single_digit:
  LDX #$DB
  STX PPUDATA
  CLC
  ADC #$DB
  STA PPUDATA
  RTS

@p2_zero_score:
  LDA #$DB       ; 0
  STA PPUDATA   
  LDA #$DB       ; 0
  STA PPUDATA
  RTS
; end update_p2_score

; @
; @ update_score
; @ Updates the scoreboards for both players
; @
update_score:
  LDA game_flags          ; do we need to update score?
  AND #SCORE_UPDATE
  BEQ end_score           ; if not, skip this

  JSR clear_update_score_flag

  ;; Prepare PPU
  JSR disable_rendering
  JSR update_p1_score
  jsr update_p2_score

end_score:
  RTS
; end update_score

; @
; @ set_update_message_flag
; @ sets the flag to update the message
; @
set_update_message_flag:
  LDA game_flags
  ORA #MESSAGE_UPDATE
  STA game_flags
  RTS
; end set_update_message_flag

; @
; @ clear_update_message_flag
; @ clears the flag to update the message
; @
clear_update_message_flag:
  LDA game_flags
  AND #CLEAR_MESSAGE
  STA game_flags
  RTS
; end clear_update_message_flag

; @
; @ start_message
; @ Draws the game start message
; @
start_message:
  LDA game_flags          ; do we need to update message?
  AND #MESSAGE_UPDATE
  BEQ end_start_message   ; if not, skip all this

  JSR clear_update_message_flag

  JSR disable_rendering
  ; Set the PPU Address to the position for the message
  LDA PPUSTATUS
  LDA #$21
  STA PPUADDR
  LDA #$6C
  STA PPUADDR

  ; If ball is in play, clear the message
  LDA game_flags
  AND #BALL_STATE_PLAY
  BNE @no_message

@show_message:
  LDX #$00
@show_message_loop:
  LDA start_message_tiles, x
  STA PPUDATA
  INX
  CPX #$08
  BNE @show_message_loop

  JMP end_start_message

@no_message:
  LDX #$00
  LDA #$00
@no_message_loop:
  STA PPUDATA
  INX
  CPX #$08
  BNE @no_message_loop
end_start_message:
  RTS
; end start_message

; @
; @ load_title
; @ Loads the title screen graphics
; @
load_title:
  LDA title_loaded          ; check if title screen is already loaded
  CMP #$01
  BNE @start                ; if it is, return from sub
  RTS
@start:
  INC title_loaded          ; set title loaded flag

  JSR disable_rendering
  JSR ready_ppu_background

  LDA #<title_screen_cmp_data
  STA pointer_lo
  LDA #>title_screen_cmp_data
  STA pointer_hi
  LDX pointer_lo
  LDY pointer_hi

  JSR decodeRLE

@load_attributes:
  JSR ready_ppu_attributes
  LDX #$00
@loop:
  LDA title_attributes, x
  STA PPUDATA
  INX
  CPX #$40                ; copying 64 ($40) bytes of data
  BNE @loop

  RTS
; end load_title

; @
; @ game_over_screen
; @ Draws the game over screen
; @
game_over_screen:
  LDA game_over_loaded
  CMP #$01
  BNE @start
  RTS

@start:
  INC game_over_loaded

  JSR disable_rendering

  ; Set the PPU Address to the position to display GAME OVER
  LDA PPUSTATUS
  LDA #$21
  STA PPUADDR
  LDA #$6B
  STA PPUADDR

; draw GAME OVER
  LDX #$00
@game_over_loop:
  LDA game_over_tiles, x
  STA PPUDATA
  INX
  CPX #$0A
  BNE @game_over_loop

  ; Set the PPU Address to the position to display PLAYER X WINS!
  LDA PPUSTATUS
  LDA #$21
  STA PPUADDR
  LDA #$A8
  STA PPUADDR

; draw PLAYER
  LDX #$00
@player_loop:
  LDA player_tiles, x
  STA PPUDATA
  INX
  CPX #$07
  BNE @player_loop

  LDX #$00
  LDA score_p1
  CMP score_p2
  BCC @two_loop
@one_loop:
  LDA one_tiles, x
  STA PPUDATA
  INX
  CPX #$03
  BNE @one_loop
  JMP @draw_wins
@two_loop:
  LDA two_tiles, x
  STA PPUDATA
  INX
  CPX #$03
  BNE @two_loop

@draw_wins:
  LDX #$0
@wins_loop:
  LDA wins_tiles, x
  STA PPUDATA
  INX
  CPX #$06
  BNE @wins_loop

@draw_press_start:
  ; Set the PPU Address to the position to display PRESS START
  LDA PPUSTATUS
  LDA #$22
  STA PPUADDR
  LDA #$6A
  STA PPUADDR

  LDX #$0
@press_start_loop:
  LDA press_start_tiles, x
  STA PPUDATA
  INX
  CPX #$0C
  BNE @press_start_loop

  RTS
; end game_over_screen

; A B SEL START UP DOWN LEFT RIGHT
; A   #%10000000
; B   #%01000000
; SEL #%00100000
; STA #%00010000
; UP  #%00001000
; DN  #%00000100
; LEF #%00000010
; RT  #%00000001
read_controller_1:
  LDA control1_buttons
  STA control_1_old_buttons
  LDA #$01      ; latch buttons
  STA CONTROL1
  LDA #$00
  STA CONTROL1
  LDX #$08      ; loop 8 times
@loop1:
  LDA CONTROL1
  LSR a
  ROL control1_buttons
  DEX
  BNE @loop1
  RTS

read_controller_2:
  LDA #$01
  STA CONTROL1
  LDA #$00
  STA CONTROL1
  LDX #$08
@loop2:
  LDA CONTROL2
  LSR a
  ROL control2_buttons
  DEX
  BNE @loop2
  RTS

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

palette_data:
	; Background palette
	.byte $0F, $2A, $2A, $2A
	.byte	$22, $36, $17, $0F
	.byte	$22, $30, $21, $0F
	.byte	$22, $27, $17, $0F
	; Sprite palette
	.byte	$0F, $2A, $19, $2A
	.byte	$0F, $20, $10, $20
 	.byte	$22, $02, $38, $3C
	.byte	$22, $1C, $15, $14

sprite_data:
  ; YCoord, tile number, attr, XCoord
  
  ;; Player 1 paddle
  .byte $FF, $02, SPR_NORMAL, $00 ; sprite 0
  .byte $FF, $02, SPR_NORMAL, $00 ; sprite 1
  .byte $FF, $02, SPR_NORMAL, $00 ; sprite 2
  .byte $FF, $02, SPR_NORMAL, $00 ; sprite 3
  
  ;; Player 2 paddle
  .byte $FF, $02, SPR_FLIP_H, $F8 ; sprite 4
  .byte $FF, $02, SPR_FLIP_H, $F8 ; sprite 5
  .byte $FF, $02, SPR_FLIP_H, $F8 ; sprite 6
  .byte $FF, $02, SPR_FLIP_H, $F8 ; sprite 7

  ;; Ball
  .byte $FF, $00, SPR_PAL_1, $80 ; sprite 8

  ;; Cursor
  .byte $FF, $01, SPR_NORMAL, $00

title_attributes:
  .repeat 8
  .byte %00000000, %00000000, %00000000, %00000000, %00000000, %00000000, %00000000, %00000000
  .endrepeat

start_message_tiles:
  .byte $F4,$F6,$E9,$F7,$F7,$00,$E5,$FF     ; PRESS A!

game_over_tiles:
  .byte $EB,$E5,$F1,$E9,$00,$00,$F3,$FA,$E9,$F6 ; GAME  OVER

player_tiles:
  .byte $F4,$F0,$E5,$FD,$E9,$F6,$00   ; PLAYER_

one_tiles:
  .byte $F3,$F2,$E9                   ; ONE

two_tiles:
  .byte $F8,$FB,$F3                   ; TWO

wins_tiles:
  .byte $00,$FB,$ED,$F2,$F7,$FF       ; _WINS!

press_start_tiles:
  .byte $F4,$F6,$E9,$F7,$F7,$00,$00,$F7,$F8,$E5,$F6,$F8 ; PRESS  START

background:
  .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00 ; row 1
  .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00 ; all sky ( $00 = sky )

  .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00 ; row 2
  .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00 ; all sky ( $00 = sky )

  .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00 ; row 3
  .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00 ; all sky ( $00 = sky )

  .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00 ; row 4
  .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00 ; all sky ( $24 = sky )

attribute_data:
	.byte	%00000000, %00000000, %00000000, %00000000
	.byte	%00000000, %00000000, %00000000, %00000000

title_screen_cmp_data:
	.byte $a8,$00,$a8,$60,$01,$02,$03,$00,$a8,$1c,$04,$05,$a8,$03,$06,$07
	.byte $07,$08,$09,$09,$0a,$0b,$0b,$0c,$0d,$a8,$02,$0e,$0f,$0f,$10,$11
	.byte $11,$12,$02,$02,$13,$00,$a8,$03,$04,$05,$a8,$1b,$14,$00,$00,$04
	.byte $05,$a8,$17,$15,$16,$05,$05,$14,$00,$00,$04,$05,$05,$17,$18,$05
	.byte $a8,$0a,$19,$1a,$05,$a8,$02,$1b,$1c,$05,$05,$14,$1d,$05,$05,$14
	.byte $00,$00,$04,$05,$05,$14,$00,$1e,$1f,$05,$05,$20,$21,$22,$23,$24
	.byte $25,$26,$27,$28,$05,$05,$29,$00,$2a,$05,$05,$2b,$2c,$05,$05,$14
	.byte $00,$00,$04,$05,$05,$2d,$00,$2e,$00,$2f,$30,$31,$32,$33,$34,$35
	.byte $00,$36,$37,$28,$05,$38,$00,$39,$04,$05,$05,$00,$3a,$05,$05,$14
	.byte $00,$00,$04,$05,$05,$3b,$00,$3c,$3d,$3e,$3f,$00,$40,$41,$00,$42
	.byte $00,$43,$44,$28,$45,$46,$47,$48,$49,$05,$4a,$00,$4b,$05,$05,$14
	.byte $00,$00,$04,$05,$05,$3b,$00,$4c,$4d,$4e,$4f,$50,$05,$51,$00,$52
	.byte $00,$00,$53,$28,$54,$00,$55,$56,$57,$58,$54,$00,$59,$05,$05,$14
	.byte $00,$00,$04,$05,$05,$3b,$00,$00,$5a,$5b,$27,$04,$05,$5c,$00,$5d
	.byte $00,$5e,$00,$28,$5f,$00,$60,$00,$00,$4b,$51,$00,$61,$05,$05,$14
	.byte $00,$00,$04,$05,$05,$62,$00,$63,$64,$05,$65,$00,$66,$67,$68,$69
	.byte $00,$6a,$03,$6b,$6c,$00,$6d,$6e,$00,$6f,$70,$0d,$71,$05,$05,$14
	.byte $00,$00,$04,$05,$05,$72,$00,$73,$05,$05,$74,$75,$76,$00,$77,$78
	.byte $00,$79,$7a,$7b,$7c,$7d,$7e,$7f,$01,$80,$81,$82,$83,$05,$05,$14
	.byte $00,$00,$04,$05,$05,$84,$85,$86,$05,$a8,$02,$87,$88,$89,$8a,$8b
	.byte $02,$8c,$8d,$8e,$05,$8f,$90,$01,$91,$05,$92,$93,$94,$05,$05,$14
	.byte $00,$00,$04,$05,$a8,$1b,$14,$00,$00,$04,$05,$a8,$18,$95,$95,$96
	.byte $97,$00,$00,$04,$05,$a8,$04,$98,$95,$99,$9a,$9a,$9b,$9c,$9d,$9e
	.byte $9e,$82,$9f,$a0,$a1,$a1,$a2,$a3,$a4,$00,$a8,$07,$a5,$a6,$a3,$a7
	.byte $00,$a8,$86,$dc,$00,$f4,$f0,$e5,$fd,$e9,$f6,$00,$a8,$37,$dd,$00
	.byte $f4,$f0,$e5,$fd,$e9,$f6,$00,$a8,$8b,$f0,$00,$a8,$06,$ff,$a8,$1d
	.byte $3f,$0f,$00,$a8,$16,$00,$a8,$00

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
.segment "VECTORS"
  .word  VBLANK
  .word  RESET
  .word  0

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
.segment "CHARS"
  .incbin "pong.chr"