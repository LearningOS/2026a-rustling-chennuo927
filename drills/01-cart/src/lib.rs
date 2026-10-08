//! 购物车结算器 —— 空白页训练 01
//!
//! 需求与规则见 ../README.md，行为规格见 ../tests/spec.rs。
//!
//! 这里什么都不给你：数据类型、函数签名、函数体，全部由你决定。
//!
//! 建议的动手顺序：
//!   1. 读完 tests/spec.rs，把每个函数写进去、从里面推出来。
//!   2. 在下面只写「类型定义 + 函数签名 + todo!()」，然后 cargo check。
//!   3. 编译器不再报错之后，再一个一个函数填实现。
//!
//! 待办清单（签名自己定）：
//!   [ ] pub enum Item { Book{...}, Food{...}, Toy{...} }   // 记得 derive
//!   [ ] fn subtotal(..) -> u32
//!   [ ] fn tax_cents(..) -> u32
//!   [ ] fn total(..) -> u32
//!   [ ] fn cheapest(..) -> Option<&Item>
//!   [ ] fn apply_discount(..) -> Result<(), String>
//!
//! 卡住了把完整报错原文发我，我只回答类型 / 借用 / 思路层面，不给实现。
