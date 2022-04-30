(module
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
)