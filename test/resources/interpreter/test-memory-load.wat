;; 当前测试用例需要提供内存的初始数据，数据如下：
;; /* addr: 0      */ 0x11, // 17
;; /* addr: 1      */ 0xf1, // uint8'241 == int8'-15 (-15=241-256)
;; /* addr: 2,3    */ 0x55, 0x66, // 0x6655
;; /* addr: 4,5    */ 0x80, 0x90, // 0x9080
;; /* addr: 6..13  */ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
;; /* addr: 14..21 */ 0x80, 0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0,

(module
    (func $f0 (result i32)
        (i32.const 0)
        (i32.load8_u offset=0)
    )

    ;; 测试只改变 offset 立即数
    (func $f1 (result i32 i32 i32 i32)
        (i32.const 0)
        (i32.load8_u offset=0)
        (i32.const 0)
        (i32.load8_u offset=1)
        (i32.const 0)
        (i32.load8_u offset=2)
        (i32.const 0)
        (i32.load8_u offset=3)
    )

    ;; 测试只改变地址
    (func $f2 (result i32 i32 i32 i32)
        (i32.const 0)
        (i32.load8_u offset=0)
        (i32.const 1)
        (i32.load8_u offset=0)
        (i32.const 2)
        (i32.load8_u offset=0)
        (i32.const 3)
        (i32.load8_u offset=0)
    )

    ;; 测试以符号数来加载
    (func $f3 (result i32 i32 i32 i32)
        (i32.const 0)
        (i32.load8_u offset=0) ;; 17
        (i32.const 0)
        (i32.load8_s offset=0) ;; 17
        (i32.const 1)
        (i32.load8_u offset=0) ;; 241
        (i32.const 1)
        (i32.load8_s offset=0) ;; -15
    )

    ;; 测试加载 16 位, 32 位数
    (func $4 (result i32 i32 i32 i32 i32)
        (i32.const 2)
        (i32.load16_u) ;; 0x6655
        (i32.const 2)
        (i32.load16_s) ;; 0x6655
        (i32.const 4)
        (i32.load16_u) ;; 0x9080
        (i32.const 4)
        (i32.load16_s) ;; 0xffff9080

        (i32.const 6)
        (i32.load)     ;; 32 位
    )

    ;; 测试加载 64 位数
    (func $5 (result i64 i64 i64 i64 i64 i64)
        (i32.const 6)
        (i64.load32_u) ;; 0x03020100
        (i32.const 6)
        (i64.load32_s) ;; 0x03020100
        (i32.const 14)
        (i64.load32_u) ;; 0xb0a09080
        (i32.const 14)
        (i64.load32_s) ;; 0xffffffffb0a09080

        (i32.const 6)  ;; 0x0706050403020100
        (i64.load)     ;; 64 位
        (i32.const 14) ;; 0xf0e0d0c0b0a09080
        (i64.load)     ;; 64 位
    )
)
