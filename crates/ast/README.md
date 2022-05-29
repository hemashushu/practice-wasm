# XiaoXuan VM - AST

WebAssembly Module AST.

WebAssembly 模块的语法树。

## 支持标准

支持 WebAssembly 1.0 以及：

- [x] [WASI](https://github.com/WebAssembly/WASI/blob/main/Proposals.md) [^1]
- [ ] [128-bit SIMD](https://github.com/WebAssembly/simd/blob/main/proposals/simd/SIMD.md)
- [ ] [Threads](https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md)
- [ ] [Reference types](https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md)
- [x] [Multiple results and block parameters](https://github.com/WebAssembly/multi-value/blob/master/proposals/multi-value/Overview.md)
- [ ] [Bulk memory operations](https://github.com/WebAssembly/bulk-memory-operations/blob/master/proposals/bulk-memory-operations/Overview.md)
- [ ] [Non-trapping float-to-int conversions](https://github.com/WebAssembly/nontrapping-float-to-int-conversions/blob/master/proposals/nontrapping-float-to-int-conversion/Overview.md)
- [x] [Sign-extension instructions](https://github.com/WebAssembly/sign-extension-ops/blob/master/proposals/sign-extension-ops/Overview.md)
- [ ] Exception handling
- [x] [Extended name section](https://github.com/WebAssembly/extended-name-section/blob/main/proposals/extended-name-section/Overview.md) [^2]
- [ ] [Multiple memories](https://github.com/WebAssembly/multi-memory/blob/main/proposals/multi-memory/Overview.md)


[^1]: WASI 的支持不在 parser 范围内；
[^2]: element 和 data 的名称尚未支持。
