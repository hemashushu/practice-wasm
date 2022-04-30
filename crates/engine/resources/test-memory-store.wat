(module
    (memory 1)    ;; 声明一个内存块，页面数为 1，即 64KB

    ;; 初始化数据：
    ;; addr: 0      : 0x11,
	;; addr: 8      : 0x2233				: 0x33, 0x22
	;; addr: 16     : 0x44556677			: 0x77, 0x66, 0x55, 0x44
	;; addr: 24     : 0xf0e0d0c0b0a09080	: 0x80, 0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0
	;; addr: 32     : "hello"  				: 0x68, 0x65, 0x6C, 0x6C, 0x6F,
	;; addr: 40		: "中文"    			 : 0xE4, 0xB8, 0xAD, 0xE6, 0x96, 0x87,

    (data 0 (offset (i32.const 0)) "\11")
    (data 0 (offset (i32.const 8)) "\33\22")
    (data 0 (offset (i32.const 16)) "\77\66\55\44")
    (data 0 (offset (i32.const 24)) "\80\90\a0\b0\c0\d0\e0\f0")
    (data 0 (offset (i32.const 32)) "hello")
    (data 0 (offset (i32.const 40)) "中文")

    (func $f0 (result i32 i32 i32)
        ;; 读第 1 个数
        (i32.const 0)
        (i32.load8_u)

        ;; 读第 2 个数
        (i32.const 8)
        (i32.load16_u)

        ;; 读第 3 个数
        (i32.const 16)
        (i32.load)
    )

    (func $f1 (result i64 i64 i64)
        ;; 读第 4 个数
        (i32.const 24)
        (i64.load)

        ;; 读 "hello" 的第一个字符
        (i32.const 32)
        (i64.load8_u)

        ;; 读 "中文" 的 utf-8 编码后的第一个字符
        (i32.const 40)
        (i64.load8_u)
    )

    (func $f2 (result i32)
        ;; 写 uint8 0xaa 到地址 00
        (i32.const 0)
        (i32.const 0xaa)
        (i32.store8)

        ;; 写 uint8 0xbb 到地址 01
        (i32.const 0)
        (i32.const 0xbb)
        (i32.store8 offset=1)

        ;; 写 uint8 0xcc 到地址 02
        (i32.const 2)
        (i32.const 0xcc)
        (i32.store8)

        ;; 写 uint8 0xdd 到地址 03
        (i32.const 2)
        (i32.const 0xdd)
        (i32.store offset=1)

        ;; 读前 4 个字节
        (i32.const 0)
        (i32.load)
    )

    (func $f3
        (i32.const 4)
        (i32.const 0x0102)
        (i32.store16)

        (i32.const 8)
        (i32.const 0xa0a1a2a3)
        (i32.store)

        (i32.const 12)
        (i64.const 0xb0)
        (i64.store8)

        (i32.const 14)
        (i64.const 0xc0c1)
        (i64.store16)

        (i32.const 16)
        (i64.const 0xd0d1d2d3)
        (i64.store32)

        (i32.const 20)
        (i64.const 0xe0e1e2e3e4e5e6e7)
        (i64.store)
    )
)
