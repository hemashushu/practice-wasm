(module
    (func $pow (param $base i32) (param $exponent i32) (result i32)
        (local $result i32)

        (local.get $base)
        (local.set $result)             ;; let $result = $base

        (loop
            (local.get $result)

            (local.get $exponent)
            (i32.const 1)
            (i32.le_s)
            (br_if 1)                   ;; if $exponent <= 1 then break $result;

            (local.get $base)
            (i32.mul)
            (local.set $result)         ;; result = result * base

            (local.get $exponent)
            (i32.const 1)
            (i32.sub)
            (local.set $exponent)       ;; $exponent = $exponent - 1

            (br 0)                      ;; loop
        )

        (local.get $result)
    )

    (export "pow" (func $pow))
)
