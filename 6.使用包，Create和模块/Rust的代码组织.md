# RUST的代码组织

## 代码组织主要包括：
- 哪些细节可以包括
- 作用域内哪些名称有效
- ...

## 模块系统
- Packet(包): Cargo的特性，让你构建，测试，共享crate
- Crate(单元包):一个模块树，它可产生一个library或可执行文件
- Module(模块)，use:让你控制代码的组织，作用域私有路径
- Path(路径): 为struct，function或module等项目的命名方式

## use关键字

use的惯用用法
函数：将函数的父级模块引入作用域（指定到父级）

struct，enum，其他：指定完整路径（指定到本身）

同名条目指定到父级
```rust
use std::fmt::Result;
use std::io::Result;
    ||
use std::fmt;
use std::io;
```

嵌套清理大量use语句

可使用嵌套路径在同一行内将上述条目进行引入:―路径相同的部分::{路径差异的部分}
```rust
use std::{fmt::Result, io::Result};

// use std::io;
// use std::io::Write;
use std::io::{self, Write};

```


## as关键字

as关键字可以为引入的路径指定本地的别名
```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```


## 通配符*

引入路径中所有的公共条目
