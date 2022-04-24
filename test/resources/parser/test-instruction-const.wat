(module
	(func
		(f32.const 12.3)
		(f32.const 45.6)
		(f32.add)
		(i32.trunc_sat_f32_s)
		(drop)
	)
)

;; 0x0015 | 10          | size of function
;; 0x0016 | 00          | 0 local blocks
;; 0x0017 | 43 cd cc 44 | F32Const { value: Ieee32(1095027917) }
;;        | 41
;; 0x001c | 43 66 66 36 | F32Const { value: Ieee32(1110861414) }
;;        | 42
;; 0x0021 | 92          | F32Add
;; 0x0022 | fc 00       | I32TruncSatF32S
;; 0x0024 | 1a          | Drop
;; 0x0025 | 0b          | End