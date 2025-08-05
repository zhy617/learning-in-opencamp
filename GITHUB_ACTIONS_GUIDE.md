# GitHub Actions 自动评测指南

本仓库配置了自动化的GitHub Actions工作流，可以自动评测Rust和C++练习并提交成绩到OpenCamp排行榜。

## 功能特性

### 支持的课程
1. **Rust练习** (rustlings) - 课程ID: 1885
2. **C++课程** (learning-cxx) - 课程ID: 1886

### 自动化流程
- 当你推送代码到main或dev分支时，GitHub Actions会自动触发
- 系统会检测你的练习目录结构
- 自动运行相应的评测命令
- 解析测试结果并生成JSON格式的成绩报告
- 自动提交成绩到OpenCamp排行榜

## 使用方法

### 1. Fork本仓库
点击右上角的"Fork"按钮，将仓库复制到你的GitHub账户下。

### 2. 完成练习
- **Rust练习**: 在`exercises/rustlings/`目录下完成练习
- **C++练习**: 在`exercises/learning-cxx/`目录下完成练习

### 3. 提交代码
```bash
git add .
git commit -m "完成练习"
git push origin main
```

### 4. 查看结果
- 推送后，GitHub Actions会自动运行
- 在仓库的"Actions"标签页可以查看运行状态
- 成绩会自动提交到OpenCamp排行榜
- 你可以在排行榜上查看自己的分数和排名

## 评测命令

### Rust课程评测
```bash
cargo xtask eval --course rustlings
```

### C++课程评测
```bash
cargo xtask eval --course learning-cxx
```

## 工作流配置

工作流文件位于`.github/workflows/test.yml`，包含以下主要步骤：

1. **初始化检查**: 检测练习目录是否存在
2. **环境设置**: 配置Rust工具链和依赖
3. **运行评测**: 执行相应的评测命令
4. **结果解析**: 解析测试输出并生成JSON报告
5. **成绩提交**: 向OpenCamp API提交成绩数据

## 排行榜查看

完成练习并推送代码后，你的成绩会自动出现在OpenCamp课程排行榜上，你可以：
- 查看自己的得分和总分
- 查看在班级中的排名
- 跟踪学习进度

## 注意事项

- 确保你的练习代码能够通过测试
- 推送到main或dev分支才会触发自动评测
- GitHub Actions运行需要几分钟时间
- 如果评测失败，请检查Actions日志中的错误信息

## 技术细节

- 使用GitHub Actions进行CI/CD
- 支持Rust和C++两种编程语言
- 自动解析测试结果并格式化为JSON
- 通过HTTP API提交成绩到远程服务器
- 支持多课程并行评测