use anyhow::{Context, Result};
use clap::Args;
use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Args)]
pub struct LearnArgs {
    /// 课程名称
    course: String,
    /// 传入 fork 仓库地址，以 git submodule 方式配置
    #[clap(long)]
    submodule: Option<String>,
}

impl LearnArgs {
    pub fn learn(self) {
        if let Err(e) = self.run_learn() {
            eprintln!("{} {}", "配置课程失败:".red().bold(), e);
        }
    }

    fn run_learn(&self) -> Result<()> {
        println!("{} {}", "开始配置课程:".blue().bold(), self.course);

        // 确保exercises目录存在
        let exercises_dir = Path::new("exercises");
        if !exercises_dir.exists() {
            fs::create_dir_all(exercises_dir).context("创建exercises目录失败")?;
        }

        // 如果提供了子模块地址，则克隆仓库
        if let Some(repo_url) = &self.submodule {
            println!("{} {}", "克隆仓库:".blue().bold(), repo_url);

            let course_dir = exercises_dir.join(&self.course);
            let submodule_path = format!("exercises/{}", self.course);

            // 检查子模块是否已在 .gitmodules 中配置
            let gitmodules_path = Path::new(".gitmodules");
            let submodule_exists_in_config = if gitmodules_path.exists() {
                let content =
                    fs::read_to_string(gitmodules_path).context("读取 .gitmodules 文件失败")?;
                content.contains(&format!("path = {}", submodule_path))
            } else {
                false
            };

            if submodule_exists_in_config {
                println!(
                    "{} {}",
                    "子模块已配置，更新到最新版本".yellow().bold(),
                    self.course
                );

                // 如果目录存在，先删除以确保完全重新克隆
                if course_dir.exists() {
                    fs::remove_dir_all(&course_dir)
                        .context(format!("删除已存在的目录 {} 失败", course_dir.display()))?;
                }

                // 初始化并更新子模块到最新版本，并设置为跟踪远程分支
                let status = Command::new("git")
                    .args(["submodule", "update", "--init", "--remote", &submodule_path])
                    .status()
                    .context("执行git submodule update命令失败")?;

                if !status.success() {
                    return Err(anyhow::anyhow!("git submodule update命令执行失败"));
                }

                // 切换到主分支并设置跟踪远程分支
                let status = Command::new("git")
                    .current_dir(&course_dir)
                    .args(["checkout", "-B", "master", "origin/master"])
                    .status()
                    .context("切换到master分支失败")?;

                if !status.success() {
                    // 如果master不存在，尝试main分支
                    let status = Command::new("git")
                        .current_dir(&course_dir)
                        .args(["checkout", "-B", "main", "origin/main"])
                        .status()
                        .context("切换到main分支失败")?;

                    if !status.success() {
                        println!(
                            "{}",
                            "警告: 无法切换到主分支，子模块将保持在detached HEAD状态".yellow()
                        );
                    }
                }
            } else {
                // 如果目录存在但子模块未配置，先删除目录
                if course_dir.exists() {
                    println!(
                        "{} {}",
                        "警告:".yellow().bold(),
                        format!("目录 {} 已存在，将被覆盖", course_dir.display())
                    );
                    fs::remove_dir_all(&course_dir)
                        .context(format!("删除已存在的目录 {} 失败", course_dir.display()))?;
                }

                // 添加新的子模块
                let status = Command::new("git")
                    .args(["submodule", "add", repo_url, &submodule_path])
                    .status()
                    .context("执行git submodule add命令失败")?;

                if !status.success() {
                    return Err(anyhow::anyhow!("git submodule add命令执行失败"));
                }

                // 确保子模块是最新的并切换到主分支
                let status = Command::new("git")
                    .args(["submodule", "update", "--init", "--remote", &submodule_path])
                    .status()
                    .context("执行git submodule update命令失败")?;

                if !status.success() {
                    return Err(anyhow::anyhow!("初始化子模块到最新版本失败"));
                }

                // 切换到主分支并设置跟踪远程分支
                let status = Command::new("git")
                    .current_dir(&course_dir)
                    .args(["checkout", "-B", "master", "origin/master"])
                    .status()
                    .context("切换到master分支失败")?;

                if !status.success() {
                    // 如果master不存在，尝试main分支
                    let status = Command::new("git")
                        .current_dir(&course_dir)
                        .args(["checkout", "-B", "main", "origin/main"])
                        .status()
                        .context("切换到main分支失败")?;

                    if !status.success() {
                        println!(
                            "{}",
                            "警告: 无法切换到主分支，子模块将保持在detached HEAD状态".yellow()
                        );
                    }
                }
            }

            println!("{} {}", "成功配置课程:".green().bold(), self.course);
            println!(
                "{} {}",
                "练习已克隆到:".green(),
                format!("exercises/{}", self.course)
            );
            println!(
                "{}",
                "你现在可以使用 'cargo xtask eval' 命令来评测练习".blue()
            );
        } else {
            println!(
                "{}",
                "未提供仓库地址，请使用 --submodule 参数指定仓库地址".yellow()
            );
        }

        Ok(())
    }
}
