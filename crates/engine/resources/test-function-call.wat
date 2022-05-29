(module
    ;; call max
    (func $0 (param i32 i32) (result i32)
        (local.get 0)
        (local.get 1)
        (call $max)
    )

    (func $1 (param i32) (result i32)
        (local.get 0)
        (call $abs)
    )

    (func $2 (param i32 i32) (result i32)
        (local.get 0)
        (local.get 1)
        (call $abs_max)
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
