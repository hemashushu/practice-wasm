(module
    (func $f0 (result i32)
        (i32.const 28)      ;; 0001_1100 （注：前面还有 24 个 0）
        (i32.clz)           ;; 27
    )

    (func $f1 (result i32)
        (i32.const 28)      ;; 0001_1100 （注：前面还有 24 个 0）
        (i32.ctz)           ;; 2
    )

    (func $f2 (result i32)
        (i32.const 28)      ;; 0001_1100 （注：前面还有 24 个 0）
        (i32.popcnt)        ;; 3
    )

    ;; 浮点数的一元运算

    (func $f3 (result f32)
        (f32.const 2.718)
        (f32.abs)           ;; 2.718
    )

    (func $f4 (result f32)
        (f32.const -2.718)
        (f32.abs)           ;; 2.718
    )

    (func $f5 (result f32)
        (f32.const 2.718)
        (f32.neg)           ;; -2.718
    )

    (func $f6 (result f32)
        (f32.const 2.718)
        (f32.ceil)          ;; 3.0
    )

    (func $f7 (result f32)
        (f32.const 2.718)
        (f32.floor)         ;; 2.0
    )

    ;; 直接裁掉小数部分，取整

    (func $f8 (result f32)
        (f32.const 2.718)
        (f32.trunc)         ;; 2.0
    )

    ;; 就近取整（4 舍 6 入，5 奇进偶不进）

    (func $f9 (result f32)
        (f32.const 1.4)
        (f32.nearest)       ;; 1.0
    )

    (func $f10 (result f32)
        (f32.const 1.6)
        (f32.nearest)       ;; 2.0
    )

    (func $f11 (result f32)
        (f32.const 2.5)
        (f32.nearest)       ;; 2.0
    )

    (func $f12 (result f32)
        (f32.const 3.5)
        (f32.nearest)       ;; 4.0
    )

    (func $f13 (result f32)
        (f32.const 25.0)
        (f32.sqrt)          ;; 5.0
    )
)
