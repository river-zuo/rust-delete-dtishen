# delete-dtishen

一个用 Rust 编写的命令行工具，用于递归删除指定目录下以特定后缀结尾的文件或目录，删除前支持批量或逐个确认。

## 功能特性

- 递归扫描当前或指定目录
- 删除所有以指定后缀（默认“的替身”）结尾的文件和目录
- 支持 `--all` 参数，一次性确认全部删除
- 支持 `--suffix=后缀` 参数，自定义匹配后缀
- 删除前有确认提示，防止误删
- 命令行参数解析基于 [clap](https://github.com/clap-rs/clap)

## 安装

```sh
cargo install --path .
```

> 安装后可在终端直接使用 `delete-dtishen` 命令。

## 使用方法

### 查看帮助
```sh
delete-dtishen --help
```

### 删除当前目录下所有以“的替身”结尾的文件/目录，逐个确认
```sh
delete-dtishen
```

### 一次性确认全部删除
```sh
delete-dtishen --all
```

### 指定目录和自定义后缀
```sh
delete-dtishen ./some_dir --all --suffix=.tmp
```

## 参数说明

- `DIR`：可选，目标目录，默认为当前目录
- `--all`：可选，批量确认全部删除
- `--suffix=后缀`：可选，指定要删除的文件或目录的后缀，默认为“的替身”

## 示例

- 删除所有以“的替身”结尾的：
  ```sh
  delete-dtishen --all
  ```
- 删除所有以 `.tmp` 结尾的：
  ```sh
  delete-dtishen --all --suffix=.tmp
  ```

---

> 本工具适合批量清理 Finder 替身、临时文件、备份目录等场景。 