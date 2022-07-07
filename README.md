# 单人海量资料维查平台

![img.png](docs/assets/Logo.svg)

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;将文件拖入共享目录，之后便可在页面高效检索。服务监控文件系统，自动维护索引，索引基于DB。所说“海量”，要自备超大容量存储，像作者这里用的是ceph；并当检索效率不足时，自行另建高性能数据库。因此，理论上，只要中间件性能足够，就可支持无限量文件检索。

## 优势
- 原生共享目录，直接操作文件。
- 用户界面，直接检索。
- 检索的是DB，而非磁盘目录，性能更好。

 ## 环境
- CentOS、Alpine 等（ext4、xfs）
- macOS（APFS）
- Windows（待测试，理论上没问题）

## 构建
```shell
cd webapp
yarn install && yarn build && cd ..
cargo build --release
```

## 镜像
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;这里为直接获得共享目录能力，使用了 samba alpine 镜像，如不需要该功能，转而去使用 scratch、alpine 也是一样的。

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;可以将 nextcloud 也集成进来，只要让它们的工作目录相同，就可以统一操作和维护索引（单指 SeaFile 端索引），并共享 nextcloud 的 App 端了。

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;env、repo 根据实际情况填写：

```shell
cp target/release/SeaFile .
strip SeaFile && upx -9 SeaFile
cp profiles/$env/* ./
tar zcvf app.tar.gz favorites config.yaml
mkdir .docker && cp app.tar.gz .dockerfile .docker/ && cd .docker/
docker build --pull -f .dockerfile --build-arg APP_ENV=$env -t $repo .
```

## 挂载smb
### macOS
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Command + k，输入服务地址，并根据随后提示，输入账号、密码。
![img.png](docs/assets/macos-smb.jpg)
### Windows
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;右键，此电脑，添加一个网络位置，`\\192.168.3.120\share`。（或，开始，运行，输入地址）（结尾的 share，请根据实际情况输入）。
![img.png](docs/assets/win-smb.png)

## 未来
| 功能  |
|:---:|
| 多用户 |


---
### 参考资料
#### 界面
- https://github.com/actix/examples/tree/master/basics/static-files
- https://github.com/actix/examples/tree/master/basics/basics

#### 上传
- https://github.com/actix/examples/blob/master/forms/multipart-s3/src/utils/upload.rs
- https://github.com/actix/examples/blob/master/forms/multipart/src/main.rs

#### 下载
- https://actix.rs/docs/static-files/
- https://github.com/actix/examples/blob/master/basics/basics/src/main.rs
