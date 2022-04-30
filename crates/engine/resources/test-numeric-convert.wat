(module

    ;; 位宽截断

    (func $f0 (result i32)
        (i64.const 123)
        (i32.wrap_i64)
    )

    ;; 位宽提升

    (func $f1 (result i64)
        (i32.const 8)
        (i64.extend_i32_s)
    )

    (func $f2 (result i64)
        (i32.const 8)
        (i64.extend_i32_u)
    )

    (func $f3 (result i64)
        (i32.const -8)
        (i64.extend_i32_s)
    )

    (func $f4 (result i64)
        (i32.const -8)
        (i64.extend_i32_u)
    )

    ;; 浮点数转整数

    (func $f5 (result i32)
        (f32.const 3.14)
        (i32.trunc_f32_s)
    )

    (func $f6 (result i32)
        (f32.const 3.14)
        (i32.trunc_f32_u)
    )

    ;; 整数转浮点数

    (func $f7 (result f32)
        (i32.const 66)
        (f32.convert_i32_s)
    )

    (func $f8 (result f32)
        (i32.const 66)
        (f32.convert_i32_u)
    )

    ;; todo 其他测试
)
