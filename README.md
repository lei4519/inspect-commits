# Validate Git Push

git pre-push hook（亦可单独使用），用于 git push 时检查提交的 commits 中是否包含敏感词。

> 不支持 windows 系统

## 为什么不是 pre-commit?

- pre-commit 可以被 `--no-verify` 跳过，pre-push 无法被跳过
- commit 会很频繁，而 push 不会
- 敏感信息暴露出去才算风险，在电脑中不算

## 检测出 commit 包含敏感信息，如何修复？

使用 `git rebase -i`, 对目标 commit 进行 `edit`，详情参考 [Git 工具 - 重写历史](https://git-scm.com/book/zh/v2/Git-%E5%B7%A5%E5%85%B7-%E9%87%8D%E5%86%99%E5%8E%86%E5%8F%B2)

⚠️ 如果包含敏感信息的 commit 已经提交到了远程仓库，重写 commit 后强制推送也没有用。只要 commit 到了远程仓库就会被永久记录，唯一的办法就是删除并重建远程仓库。

## 安装
```sh
git clone git@github.com:lei4519/validate-git-push.git $HOME/.validate-git-push && $HOME/.validate-git-push/scripts/link
```

## 使用

## 配置

## 卸载

```sh
validate-git-push unset-global-hook && rm /usr/local/bin/validate-git-push && rm -rf ~/.validate-git-push
```

