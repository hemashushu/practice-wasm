# XiaoXuan VM - Engine

An interpreter for executing WebAssembly instructions.

一个解析型的 WebAssembly 指令执行器。

## 支持标准

支持 WebAssembly 1.0 (20191205) 以及：

- [x] [WASI](https://github.com/WebAssembly/WASI/blob/main/Proposals.md)
- [ ] [128-bit SIMD](https://github.com/WebAssembly/simd/blob/main/proposals/simd/SIMD.md)
- [ ] [Threads](https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md)
- [x] [Multiple results and block parameters](https://github.com/WebAssembly/multi-value/blob/master/proposals/multi-value/Overview.md)
      函数及流程控制结构块（即 `block`、`loop` 和 `if`）支持多返回值。
- [ ] [Bulk memory operations](https://github.com/WebAssembly/bulk-memory-operations/blob/master/proposals/bulk-memory-operations/Overview.md)
      - 添加了如下指令：
        * `memory.fill`
        * `memory.init`
        * `memory.copy`
        * `data.drop`
        * `table.init`
        * `table.copy`
        * `elem.drop`
      - `element` 和 `data` 项目添加了属性 `passive`
- [ ] [Reference types](https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md)
      - `引用类型` 扩展为 `funcref` and `externref`
      - 数据类型添加多了 `引用类型`
      - 添加了下列指令：
        * `ref.null`
        * `ref.func`
        * `ref.is_null`
        * `table.fill`
        * `table.get`
        * `table.grow`
        * `table.set`
        * `table.size`
      - 一个模块允许多个 `table`
      - 指令 `call_indirect` 的表索引值可以非零
      - `elem` 段里面的项目的表索引值可以非零
- [ ] [Non-trapping float-to-int conversions](https://github.com/WebAssembly/nontrapping-float-to-int-conversions/blob/master/proposals/nontrapping-float-to-int-conversion/Overview.md)
      - 用于将浮点数转换为整数，NaN 转为 0，正负无穷转为最大最小值，不会抛出异常。
      - 添加了下列指令：
        * `i32.trunc_sat_f32_s`
        * `i32.trunc_sat_f32_u`
        * `i32.trunc_sat_f64_s`
        * `i32.trunc_sat_f64_u`
        * `i64.trunc_sat_f32_s`
        * `i64.trunc_sat_f32_u`
        * `i64.trunc_sat_f64_s`
        * `i64.trunc_sat_f64_u`

- [x] [Sign-extension instructions](https://github.com/WebAssembly/sign-extension-ops/blob/master/proposals/sign-extension-ops/Overview.md)
- [ ] [Exception handling](https://github.com/WebAssembly/exception-handling/blob/main/proposals/exception-handling/Exceptions.md)
- [x] [Extended name section](https://github.com/WebAssembly/extended-name-section/blob/main/proposals/extended-name-section/Overview.md)
- [ ] [Multiple memories](https://github.com/WebAssembly/multi-memory/blob/main/proposals/multi-memory/Overview.md)
- [x] [Sign-extension operators](https://github.com/WebAssembly/spec/blob/main/proposals/sign-extension-ops/Overview.md)
      添加了如下指令：
      * `i32.extend8_s`
      * `i32.extend16_s`
      * `i64.extend8_s`
      * `i64.extend16_s`
      * `i64.extend32_s`
