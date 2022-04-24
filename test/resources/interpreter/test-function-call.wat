(module
    (func $0 (result i32)
        (f32.const 2.718)   ;; 2.718
        (f32.const 3.142)   ;; 3.142
        (call $max)         ;; 3.142
        (i32.trunc_f32_s)   ;; 3
    )

    (func $1 (result i32)
        (f32.const 1.1)     ;; 1.1

        (f32.const 5.5)
        (f32.neg)           ;; -5.5

        (call $max)         ;; 1.1
        (i32.trunc_f32_s)   ;; 1
    )

    (func $2 (result i32)
        (f32.const 1.1)     ;; 1.1

        (f32.const 5.5)
        (f32.neg)           ;; -5.5

        (call $abs_max)     ;; -5.5
        (i32.trunc_f32_s)   ;; -5
    )

    (func $abs (param $x f32) (result f32)
        (local.get $x)
        (f32.abs)
    )

    (func $max (param f32 f32) (result f32)
        (local.get 0) ;; selct by 1
        (local.get 1) ;; selct by 0

        (local.get 0) ;; lhs
        (local.get 1) ;; rhs
        (f32.gt)

        (select)
    )

    (func $abs_max (param f32 f32) (result f32)
        (local.get 0) ;; selct by 1
        (local.get 1) ;; selct by 0

        (local.get 0) ;; lhs
        (call $abs)

        (local.get 1) ;; rhs
        (call $abs)

        (f32.gt)

        (select)
    )
)
