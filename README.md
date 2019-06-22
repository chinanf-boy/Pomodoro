| 源                                       |
| ---------------------------------------- |
| https://github.com/PrismaPhonic/Pomodoro |

# 番茄钟（本项目 fork 而来，但并没有与源合并）

这个箱子为您提供终端上的番茄钟。

## 下载与构建

```bash
git clone https://github.com/chinanf-boy/Pomodoro.git
cd Pomodoro
# 运行
cargo run -- -w 1 -s 1
# 构建
cargo build --release
```

> 最好选择 build 版本，复制项目目录下生成的`./target/release/pomodoro` 到 终端（的环境变量）接触的地方。

## 使用番茄钟

要使用，只需运行它。默认情况下，它会给你 25 分钟的工作时间，5 分钟的短休息和 20 分钟的长休息。

```terminal
$ pomodoro
```

您可以传递终端标志来自定义时间。`-w`会设置工作时间，`-s`将设置短休息时间，而`-l`将设置长休息时间。这是一个设置自定义番茄钟的例子，工作时间为 30 分钟，短休息时间为 10 分钟，长休息时间为 25 分钟：

```terminal
$ pomodoro -w 30 -s 10 -l 25
```

## 相关

Cargo.toml

- `termion = "1.5.1"` 终端输出的控制
- `rodio = "0.9.0"` 音频播放
- `rust-embed="4.3.0"` 文件嵌入二进制
- `structopt = "0.2.14"` 命令行参数结构宏
