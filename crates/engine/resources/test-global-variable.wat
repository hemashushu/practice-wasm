(module
    (global $g0 i32 (i32.const 55))
    (global $g1 (mut i32) (i32.const 66))

    ;; callme with ()
    (func $f0 (result i32 i32)      ;; (55, 66)
        (global.get $g0)
        (global.get $g1)

        ;; --- top  ---
        ;; $g1
        ;; $g0
        ;; -- bottom --
    )

    ;; call me with (3,4)
    (func $f1 (param i32 i32) (result i32)  ;; -12
        (local.get 0)       ;; 3
        (local.get 1)       ;; 4
        (global.get $g0)    ;; 55
        (global.get $g1)    ;; 66

        (i32.sub)           ;; 55 - 66 = -11
        (i32.sub)           ;; 4 - (-11) = 15
        (i32.sub)           ;; 3 - 15 = -12
    )

    ;; call me with (7)
    (func $f2 (param $a i32) (result i32)   ;; 114
        (local $z i32)      ;; index 1

        (global.get $g1)    ;; 66
        (local.set $z)      ;; 66

        (local.get $a)      ;; 7
        (global.set $g1)    ;; 7

        (global.get $g0)    ;; 55
        (global.get $g1)    ;; 7
        (local.get $z)      ;; 66

        (i32.sub)           ;; 7 - 66 = -59
        (i32.sub)           ;; 55 - (-59) = 114
    )
)
