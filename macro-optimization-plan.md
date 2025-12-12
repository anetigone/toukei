# Toukei 语言定义映射宏优化方案

## 问题分析

### 当前状态
- `src/langs/registry.rs` 中有64个重复的 `map.insert()` 调用
- 每个调用都是 `LangType::Variant => &DEF_NAME` 的模式
- 代码冗余、维护困难、容易出错
- 添加新语言类型需要在多处修改代码

### 目标
1. 消除重复代码，提高可维护性
2. 保持类型安全和性能
3. 提升代码可读性
4. 简化新语言类型的添加过程
5. 不引入外部依赖（除非必要）

## 解决方案设计

### 方案一：声明式宏（推荐）

#### 1. 创建语言映射宏
在 `src/langs/macros.rs` 中定义：

```rust
/// 创建语言类型到定义的HashMap映射宏
/// 使用方式：
/// ```
/// use super::definitions::*;
///
/// lang_def_map! {
///     Asciidoc => ASCIIDOC,
///     Astro => ASTRO,
///     C => C,
///     // ... 其他语言映射
///     // 支持注释:
///     H => C,        // 使用C定义处理.h文件
///     Hpp => CPP,     // 使用C++定义处理.hpp文件
///     Xml => HTML,    // 使用HTML定义处理XML文件
///     Shell => RUST,  // 基础Shell定义，可后续改进
///     Perl => RUBY,   // 基础Perl定义，可后续改进
///     Text => MARKDOWN // 使用Markdown定义处理Text文件
/// }
/// ```
macro_rules! lang_def_map {
    (
        $($lang_type:ident => $lang_def:ident),* $(,)?
    ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert(crate::langs::lang_type::LangType::$lang_type, &super::definitions::$lang_def);
            )*
            map
        }
    };
}

/// 验证语言映射完整性的宏
/// 确保所有LangType枚举值都有对应的定义
macro_rules! ensure_all_langs_mapped {
    () => {
        const _: () = {
            #[allow(dead_code)]
            fn verify_all_lang_types_have_definitions() {
                use crate::langs::lang_type::LangType;
                use crate::langs::definitions::*;

                // 编译时检查：确保每个枚举值都能映射
                match LangType::Asciidoc {
                    LangType::Asciidoc => &ASCIIDOC,
                    LangType::Astro => &ASTRO,
                    LangType::C => &C,
                    LangType::Clojure => &CLOJURE,
                    LangType::Cpp => &CPP,
                    LangType::Csharp => &CSHARP,
                    LangType::Css => &CSS,
                    LangType::D => &D,
                    LangType::Dart => &DART,
                    LangType::Elm => &ELM,
                    LangType::Erlang => &ERLANG,
                    LangType::Fsharp => &FSHARP,
                    LangType::Go => &GO,
                    LangType::Graphql => &GRAPHQL,
                    LangType::H => &C,
                    LangType::Hpp => &CPP,
                    LangType::Haskell => &HASKELL,
                    LangType::Html => &HTML,
                    LangType::Java => &JAVA,
                    LangType::Javascript => &JAVASCRIPT,
                    LangType::Json => &JSON,
                    LangType::Jsonnet => &JSONNET,
                    LangType::Julia => &JULIA,
                    LangType::Kotlin => &KOTLIN,
                    LangType::Lua => &LUA,
                    LangType::Markdown => &MARKDOWN,
                    LangType::Nix => &NIX,
                    LangType::Ocaml => &OCAML,
                    LangType::Php => &PHP,
                    LangType::Python => &PYTHON,
                    LangType::Qcl => &QCL,
                    LangType::Qsharp => &QSHARP,
                    LangType::R => &R,
                    LangType::Regex => &REGEX,
                    LangType::Ruby => &RUBY,
                    LangType::Rust => &RUST,
                    LangType::Sass => &SASS,
                    LangType::Scala => &SCALA,
                    LangType::Sql => &SQL,
                    LangType::Swift => &SWIFT,
                    LangType::Tcl => &TCL,
                    LangType::Tex => &TEX,
                    LangType::Toml => &TOML,
                    LangType::Typescript => &TYPESCRIPT,
                    LangType::V => &V,
                    LangType::WenYan => &WENYAN,
                    LangType::Xml => &HTML,
                    LangType::Yaml => &YAML,
                    LangType::Zig => &ZIG,
                    LangType::Shell => &RUST,
                    LangType::Perl => &RUBY,
                    LangType::Text => &MARKDOWN,
                    LangType::Unknown => &MARKDOWN,
                }
            }
        };
    };
}
```

#### 2. 重构 registry.rs
将原来的64行重复代码替换为：

```rust
use lazy_static::lazy_static;
use strum::VariantNames;

use crate::langs::lang_type::LangType;
use crate::langs::lang_def::LangDef;
use crate::langs::definitions::*;
use super::macros::{lang_def_map, ensure_all_langs_mapped};

lazy_static! {
    pub static ref SUPPORTED_LANGUAGES: Vec<&'static str> = LangType::VARIANTS.to_vec();

    /// 语言类型到语言定义的映射
    /// 使用宏消除重复代码，提高可维护性
    pub static ref LANGUAGE_DEFINITIONS: std::collections::HashMap<LangType, &'static LangDef> =
        lang_def_map! {
            Asciidoc => ASCIIDOC,
            Astro => ASTRO,
            C => C,
            Clojure => CLOJURE,
            Cpp => CPP,
            Csharp => CSHARP,
            Css => CSS,
            D => D,
            Dart => DART,
            Elm => ELM,
            Erlang => ERLANG,
            Fsharp => FSHARP,
            Go => GO,
            Graphql => GRAPHQL,
            H => C,              // Header files use C definition
            Hpp => CPP,            // C++ headers use C++ definition
            Haskell => HASKELL,
            Html => HTML,
            Java => JAVA,
            Javascript => JAVASCRIPT,
            Json => JSON,
            Jsonnet => JSONNET,
            Julia => JULIA,
            Kotlin => KOTLIN,
            Lua => LUA,
            Markdown => MARKDOWN,
            Nix => NIX,
            Ocaml => OCAML,
            Php => PHP,
            Python => PYTHON,
            Qcl => QCL,
            Qsharp => QSHARP,
            R => R,
            Regex => REGEX,
            Ruby => RUBY,
            Rust => RUST,
            Sass => SASS,
            Scala => SCALA,
            Sql => SQL,
            Swift => SWIFT,
            Tcl => TCL,
            Tex => TEX,
            Toml => TOML,
            Typescript => TYPESCRIPT,
            V => V,
            WenYan => WENYAN,
            Xml => HTML,           // XML uses HTML-like definition
            Yaml => YAML,
            Zig => ZIG,
            Shell => RUST,         // Basic shell definition
            Perl => RUBY,         // Basic Perl definition
            Text => MARKDOWN       // Text files similar to markdown
        };

    // 编译时验证映射完整性
    ensure_all_langs_mapped!();

    /// 根据语言类型获取对应的语言定义
    pub fn get_lang_def(lang_type: &LangType) -> Option<&'static LangDef> {
        LANGUAGE_DEFINITIONS.get(lang_type).copied()
    }
}
```

#### 3. 更新模块导出
在 `src/langs/mod.rs` 中添加：

```rust
pub mod definitions;
pub mod lang_def;
pub mod lang_err;
pub mod lang_type;
pub mod macros;     // 新增宏模块
pub mod registry;
```

### 方案二：使用 maplit crate（备选）

如果愿意引入外部依赖：

```toml
# Cargo.toml
[dependencies]
maplit = "1.0"
```

```rust
use maplit::hashmap;

pub static ref LANGUAGE_DEFINITIONS: std::collections::HashMap<LangType, &'static LangDef> = hashmap! {
    LangType::Asciidoc => &ASCIIDOC,
    LangType::Astro => &ASTRO,
    // ... 其他映射
};
```

## 优势分析

### 方案一优势
1. **零外部依赖**：不增加项目依赖复杂度
2. **编译时优化**：HashMap在编译时构建，运行时性能最优
3. **类型安全**：编译器确保所有映射的类型正确性
4. **代码简洁**：将64行重复代码简化为清晰的声明式配置
5. **易于维护**：新增语言类型只需添加一个映射条目
6. **内联文档**：宏定义中包含详细的使用说明和注释

### maplit方案优势
1. **标准库支持**：使用成熟的第三方库
2. **语法简洁**：类似HashMap的字面量语法
3. **社区认可**：广泛使用的解决方案

### 劣势分析

#### 方案一
- **调试困难**：宏展开错误可能较难调试
- **学习成本**：需要理解Rust宏的语法
- **编译时间**：复杂宏可能略微增加编译时间

#### maplit方案
- **外部依赖**：增加项目依赖
- **版本锁定**：需要管理maplit版本兼容性

## 实施建议

### 首选方案：声明式宏
基于项目的当前架构和维护成本考虑，推荐使用方案一的声明式宏：

1. **最小化依赖变更**：不引入新的crate依赖
2. **保持现有功能**：完全兼容现有API
3. **提供扩展性**：为未来添加新语言类型提供简洁的机制
4. **增强类型安全**：编译时完整性检查确保映射正确

### 实施步骤
1. 创建 `src/langs/macros.rs` 文件
2. 更新 `src/langs/mod.rs` 导出宏模块
3. 重构 `src/langs/registry.rs` 使用新宏
4. 运行测试确保功能正确性
5. 添加文档说明新的代码结构

## 预期效果

### 代码行数对比
- **重构前**：64行重复的 `map.insert()` 调用
- **重构后**：约30行清晰的声明式映射（包括注释和格式化）

### 可维护性提升
- **添加新语言**：从需要在多处修改改变为只需添加一个映射条目
- **错误减少**：消除手动复制粘贴的错误风险
- **代码可读性**：清晰的配置式声明，一目了然

### 性能保证
- **编译时构建**：无额外运行时开销
- **零成本抽象**：宏展开后的代码与原手写代码等效
- **类型安全**：编译器验证所有类型关系

## 风险缓解

### 开发风险
1. **宏调试**：提供清晰的错误信息和文档
2. **测试覆盖**：添加单元测试验证映射正确性
3. **向后兼容**：确保公共API接口不变

### 维护风险
1. **文档维护**：在宏文件中提供详细的使用说明
2. **团队培训**：确保团队成员理解新的代码结构
3. **代码审查**：建立添加新语言类型的审查流程

这个方案在保持现有功能完整性的同时，显著提升了代码的可维护性和可读性，为项目的长期发展奠定了更好的基础。