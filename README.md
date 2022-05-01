# finclip-rust-demo

通过实现一个加密钱包小程序，演示如何利用Rust开发算法型、通用、安全、原生的功能组件，集成到iOS中再作为自定义API注入到FinClip SDK，供FinClip小程序使用。

## 场景
一个native iOS app，提供了加密相关的一些基本能力，可以生成和管理密钥对。此能力可以被封装为API，注入到FinClip SDK中，向其增加了加密钱包相关能力，让Web3应用以小程序方式加载运行在App上。

## 所涉及技术
涉及FinClip社区版（docker image）、Rust语言及相关工具链、FinClip IDE、FinClip小程序、iOS/Objective-C/Xcode。

## 详细介绍
参考：
- [《FinClip小程序+Rust：（一）夹心饼架构》](http://www.finclip.com/blog/finclip-rust-1)
- [《FinClip小程序+Rust：（二）环境搭建》](http://www.finclip.com/blog/finclip-rust-2)
- [《FinClip小程序+Rust：（三）一个加密钱包》](http://www.finclip.com/blog/finclip-rust-3)
- [《FinClip小程序+Rust：（四）端到端融合》](http://www.finclip.com/blog/finclip-rust-4)
- [《FinClip小程序+Rust：（五）用内联SVG生成二维码》](https://www.finclip.com/blog/finclip-rust-5/)
