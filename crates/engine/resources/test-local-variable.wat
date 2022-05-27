(module
    ;; call me with (10,20)
    (func $f0 (param $a i32) (param $b i32) (result i32 i32) ;; (10,20)
        (local.get $a)
        (local.get $b)

        ;; --- top  ---
        ;; $b
        ;; $a
        ;; -- bottom --
    )

    ;; call me with (33,44)
    (func $f1 (param i32 i32) (result i32)  ;; -11
        (local.get 0)
        (local.get 1)
        (i32.sub)           ;; 33 - 44
    )

    ;; call me with (33,22)
    (func $f2 (param $a i32) (param $b i32) (result i32)    ;; 10
        (local $x i32)      ;; index 2
        (local $y i32)      ;; index 3
        (local $z i32)      ;; index 4

        (local.get $a)
        (local.set $x)      ;; 33

        (local.get $b)
        (local.set $y)      ;; 22

        (i32.const 3)
        (local.set $a)      ;; 3

        (i32.const 4)
        (local.set $b)      ;; 4

        (local.get $a)      ;; 3
        (local.get $b)      ;; 4
        (local.get $x)      ;; 33
        (local.get $y)      ;; 22

        (i32.sub)           ;; 33 - 22 = 11
        (i32.sub)           ;; 4 - 11 = -7
        (i32.sub)           ;; 3 - (-7) = 10
    )
)
