(module
	(global $g1 i32 (i32.const 10))
	(global $g2 (mut i32) (i32.const 20))

	;; call me with (55,66)
	(func (param $a i32) (param $b i32) (result i32 i32 i32 i32)
        (local $la i32)
        (local $lb i32)
        (local i32 i32)

		(global.get $g2)
		(local.set $la)		;; $la = $g2 ($la == 20)

		(global.get $g1)
		(global.set $g2)	;; $g2 = $g1 ($g2 == 10)

		(local.get $b)
		(local.set $lb)		;; $lb = $b ($lb == 66)

		(local.get $a)		;; $b = $a ($b == 55)
		(local.set $b)

		(local.get $b)
		(local.set 2)		;; locals[2] == 55

		(i32.const 77)
		(local.set 3)		;; locals[3] == 77

		(local.get 2)		;; 55
		(local.get 3)		;; 77
		(local.get $la)		;; 20
		(local.get $lb)		;; 66

		;; global 0 == 10, global 1 == 10
		;; return [66,20,77,55]
	)
)
