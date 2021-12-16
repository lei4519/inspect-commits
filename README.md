# Validate Git Push

git pre-push hook（命令亦可单独使用），用于 git push 时检查提交的 commits 中是否包含敏感词。

> 不支持 windows 系统

![](https://gitee.com/lei451927/picture/raw/master/images/20211216134305.png)

## 安装
```sh
git clone git@github.com:lei4519/validate-git-push.git --depth=1 $HOME/.validate-git-push && $HOME/.validate-git-push/scripts/install
```

安装成功后，在终端中执行 `validate-git-push` 命令应该会有信息输出。
```sh
validate-git-push
```

## 卸载

```sh
validate-git-push unset-global-hook && rm /usr/local/bin/validate-git-push && rm -rf ~/.validate-git-push
```

## 命令

```sh
SUBCOMMANDS:
    check                检查当前分支中未提交到远程的 commits，必须传入远程仓库的名称
    checkall             检查当前分支下的所有 commits
    config               编辑配置文件；-p 参数返回文件地址
    set-global-hook      将程序配置为 git global core.hooksPath
    unset-global-hook    清除 git global core.hooksPath 配置
```


### `validate-git-push config`
编辑配置文件，文件格式为 JSON。

#### 配置签名
```ts
// 可配多个规则
type Rules = Array<Rule>

type Rule = {
	// 当前规则需要排除的远程地址
	excludes: Array<string>
	// 当前规则需要检查的敏感词
	words: Array<string>
}
```

#### 配置示例
```json
{
	"rules": [
		{
			"excludes": [
				"github.com"
			],
			"words": [
				"password"
			]
		},
		{
			"excludes": [
				"gitlab.com"
			],
			"words": [
				"sensitive"
			]
		}
	]
}
```

### `validate-git-push check`
检查当前分支中未提交到远程的 commits，必须传入远程仓库的名称

```sh
validate-git-push check origin
```

### `validate-git-push checkall`
检查当前分支下的所有 commits

### `validate-git-push set-global-hook`
将程序配置为全局的 pre-push hook，实际就是在设置 `git config --global core.hooksPath`

### `validate-git-push unset-global-hook`
清除全局 hook 配置，等同于执行 `git config --global --unset core.hooksPath`。

⚠️ 注意：
- 如果已存在全局 hook，此命令显示警告信息，并覆盖已存在的 hook。
- 如果在 git 仓库中配置了 hooksPath 会覆盖 git 全局的 hook

如果存在以上情况，可以使用 [husky](https://github.com/typicode/husky) 进行多 hook 管理。

## 为什么不是 pre-commit?

- pre-commit 可以被 `--no-verify` 跳过，pre-push 无法被跳过
- commit 会很频繁，而 push 不会
- 敏感信息暴露出去才算风险，在电脑中不算

## 检测出 commit 包含敏感信息，如何修复？

使用 `git rebase -i`, 对目标 commit 进行 `edit`，详情参考 [Git 工具 - 重写历史](https://git-scm.com/book/zh/v2/Git-%E5%B7%A5%E5%85%B7-%E9%87%8D%E5%86%99%E5%8E%86%E5%8F%B2)

⚠️  如果包含敏感信息的 commit 已经提交到了远程仓库，重写 commit 后强制推送也没有用。只要 commit 到了远程仓库就会被永久记录，唯一的办法就是删除并重建远程仓库。
