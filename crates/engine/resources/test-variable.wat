(module
    (global $g1 i32 (i32.const 10))
    (global $g2 (mut i32) (i32.const 20))

    (func $f0 (param $a i32) (param $b i32) (result i32 i32)
        (local.get $b)
        (local.get $a)

        ;; --- top  ---
        ;; $a
        ;; $b
        ;; -- bottom --
    )

    (func $f1 (param i32 i32) (result i32)
        (local.get 0)
        (local.get 1)
        (i32.add)
    )

    ;; call me with (55,66)
    (func $f2 (param $a i32) (param $b i32) (result i32 i32 i32 i32)
        (local $la i32)     ;; index 2
        (local $lb i32)     ;; index 3
        (local i32 i32)     ;; index 4,5

        (global.get $g2)    ;; 20
        (local.set $la)        ;; $la = $g2 ($la == 20)

        (global.get $g1)    ;; 10
        (global.set $g2)    ;; $g2 = $g1 ($g2 == 10)

        (global.get $g2)    ;; 10
        (local.set $lb)     ;; $lb = $g2 ($lb == 10)

        ;; ----------

        (local.get $a)      ;; 55
        (local.set 4)       ;; locals[4] == 55

        (local.get $b)      ;; 66
        (local.set $a)      ;;

        (local.get $a)      ;;
        (local.set 5)       ;; locals[5] == 66

        (local.get 4)       ;; 55
        (local.get 5)       ;; 66
        (local.get $la)     ;; 20
        (local.get $lb)     ;; 10

        ;; return [55,66,20,10]
    )
)
