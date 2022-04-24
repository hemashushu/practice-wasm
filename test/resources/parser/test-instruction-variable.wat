(module
	(global $g1 (mut i32) (i32.const 1))  ;; $g1, $g2 可视为自动索引值
	(global $g2 (mut i32) (i32.const 2))
	(func (param $a i32) (param $b i32)
        (local $la i32)
        (local $lb i32)
        (local i64 i64)
		(global.get $g1)
		(global.set $g2)
		(local.get $a)
		(local.set $b)
	)
)

;; 0x0024 | 0e          | size of function
;; 0x0025 | 02          | 2 local blocks
;; 0x0026 | 02 7f       | 2 locals of type I32
;; 0x0028 | 02 7e       | 2 locals of type I64
;; 0x002a | 23 00       | GlobalGet { global_index: 0 }
;; 0x002c | 24 01       | GlobalSet { global_index: 1 }
;; 0x002e | 20 00       | LocalGet { local_index: 0 }
;; 0x0030 | 21 01       | LocalSet { local_index: 1 }
;; 0x0032 | 0b          | End