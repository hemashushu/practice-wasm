(module
    ;; call return_8, for testing stack call frame
    (func $0 (result i32)
        (i32.const 1)
        (call $return_8)
        (i32.const 2)
    )

    ;; call max
    (func $1 (param i32 i32) (result i32)
        (local.get 0)
        (local.get 1)
        (call $max)
    )

    ;; call abs
    (func $2 (param i32) (result i32)
        (local.get 0)
        (call $abs)
    )

    ;; call abs_max
    (func $3 (param i32 i32) (result i32)
        (local.get 0)
        (local.get 1)
        (call $abs_max)
    )

    (func $return_8 (result i32)
        (i32.const 8)
    )

    (func $abs (param i32) (result i32)
        (local.get 0)   ;; a

        (i32.const 0)
        (local.get 0)
        (i32.sub)       ;; 0-a

        (local.get 0)   ;; lhs
        (i32.const 0)   ;; rhs
        (i32.gt_s)

        (select)
    )

    (func $max (param i32 i32) (result i32)
        (local.get 0)   ;; select by not 0
        (local.get 1)   ;; select by 0

        (local.get 0)   ;; lhs
        (local.get 1)   ;; rhs
        (i32.gt_s)

        (select)
    )

    (func $abs_max (param i32 i32) (result i32)
        (local.get 0)   ;; select by not 0
        (local.get 1)   ;; select by 0

        (local.get 0)   ;; lhs
        (call $abs)

        (local.get 1)   ;; rhs
        (call $abs)

        (i32.gt_s)

        (select)
    )
)
