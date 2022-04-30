(module
    (func $f0 (result i32 i32)
        (i32.const 11)
        (i32.const 22)  ;; 后弹，lhs
        (i32.const 33)  ;; 先弹，rhs
        (i32.add)       ;; 55
    )

    (func $f1 (result i32 i32)
        (i32.const 11)
        (i32.const 22)  ;; 后弹，lhs
        (i32.const 33)  ;; 先弹，rhs
        (i32.sub)       ;; -11
    )

    (func $f2 (result i32 i32)
        (i32.const 11)
        (i32.const 22)  ;; 后弹，lhs
        (i32.const 33)  ;; 先弹，rhs
        (i32.mul)       ;; 726
    )

    (func $f3 (result i32 i32)
        (i32.const 11)
        (i32.const -8)  ;; 后弹，lhs
        (i32.const 2)   ;; 先弹，rhs
        (i32.div_s)     ;; -4
    )

    (func $f4 (result i32 i32)
        (i32.const 11)
        (i32.const -8)  ;; 后弹，lhs，0b(29个'1'） + 000
        (i32.const 2)   ;; 先弹，rhs
        (i32.div_u)     ;; 0b0 + （29个'1'） + 00
    )

    (func $f5 (result i32 i32)
        (i32.const 11)
        (i32.const -8)  ;; 后弹，lhs
        (i32.const 3)   ;; 先弹，rhs
        (i32.rem_s)     ;; -2
    )

    (func $f6 (result i32 i32)
        (i32.const 11)
        (i32.const -8)  ;; 后弹，lhs
        (i32.const 3)   ;; 先弹，rhs
        (i32.rem_u)     ;;
    )

    ;; 位运算

    (func $f7 (result i32 i32)
        (i32.const 11)
        (i32.const 248) ;; 后弹，lhs, 0b1111_1000
        (i32.const 25)  ;; 先弹，rhs, 0b0001_1001
        (i32.and)       ;; 0b（24个'0'） + 0001_1000
    )

    (func $f8 (result i32 i32)
        (i32.const 11)
        (i32.const 248) ;; 后弹，lhs, 0b1111_1000
        (i32.const 25)  ;; 先弹，rhs, 0b0001_1001
        (i32.or)        ;; 0b（24个'0'） + 1111_1001
    )

    (func $f9 (result i32 i32)
        (i32.const 11)
        (i32.const 248) ;; 后弹，lhs, 0b1111_1000
        (i32.const 25)  ;; 先弹，rhs, 0b0001_1001
        (i32.xor)       ;; 0b(24个'0') + 1110_0001
    )

    (func $f10 (result i32 i32)
        (i32.const 11)
        (i32.const -1)  ;; 后弹，lhs, 32 个 1
        (i32.const 4)   ;; 先弹，rhs, 4
        (i32.shl)       ;; 0b(28个'1') + 0000
    )

    (func $f11 (result i32 i32)
        (i32.const 11)
        (i32.const -1)  ;; 后弹，lhs, 32 个 1
        (i32.const 4)   ;; 先弹，rhs, 4
        (i32.shr_s)     ;; 0b + (32个'1')
    )

    (func $f12 (result i32 i32)
        (i32.const 11)
        (i32.const -1)  ;; 后弹，lhs, 32 个 1
        (i32.const 4)   ;; 先弹，rhs, 4
        (i32.shr_u)     ;; 0b0000 + (28个'1')
    )

    ;; 左旋

    (func $f13 (result i32 i32)
        (i32.const 11)
        (i32.const -8)  ;; 后弹，lhs，0b(29个'1'） + 000
        (i32.const 2)   ;; 先弹，rhs
        (i32.rotl)      ;; 0b + （27个'1'） + 000 + 11
    )

    ;; 右旋

    (func $f14 (result i32 i32)
        (i32.const 11)
        (i32.const -8)  ;; 后弹，lhs，0b(29个'1'） + 000
        (i32.const 2)   ;; 先弹，rhs
        (i32.rotr)      ;; 0b00 + （29个'1'） + 0
    )
)
