# muse-integrator

积分账户模块中的积分交易处理，账单信息模块

## 项目启动说明

- 安装 loco

```
cargo install loco-cli

cargo install sea-orm-cli
```

本地启动需要替换 config/development.yml 文件中的 database.url 配置。 可以自行 docker 启动 mysql 和 redis 进行配置替换即可

- 项目启动

如果需要本地使用，需要搭建本地 redis，mysql。 并创建数据库。

- 执行表结构生成

```
cargo loco db migrate
```

loco 启动项目：

```
cargo loco start
```
