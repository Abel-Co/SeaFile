# 单人海量资料维查平台

![img.png](docs/assets/Logo.svg)

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;将文件拖入共享目录，之后便可在页面高效检索。服务监控文件系统，自动维护索引，索引基于DB。所说“海量”，要自备超大容量存储，像作者这里用的是ceph；并当检索效率不足时，自行另建高性能数据库。因此，理论上，只要中间件性能足够，就可支持无限量文件检索。

## 优势
- 使用原生共享目录，直接操作文件。
- 用户界面，立即可搜索。
- 检索的是DB，而非磁盘目录，性能更好。
- 每次搜索、浏览，会对索引进行异步巡视校准。 （当文件被其他途径删除，而未感知到系统事件时，索引便残留了，且可被检索到，此时再次检索，会获得校准后的结果）

 ## 环境
- CentOS、Alpine 等（ext4、xfs）（Linux 为未来的主要支持环境）
- macOS（APFS）（Finder下操作部分感知不到变更，命令行没问题）
- Windows（待测试，理论上没问题）
- Internet（是的，界面服务使用了CDN资源，这意味着，作为发起请求的客户机，需要接入互联网，而非只需要内网。如确实希望剔除对公网的依赖，需要从源码自行编译，并移除CDN相关配置）
- PostgreSQL，当前仅支持 PG 数据库。

## 构建
```shell
cd webapp
yarn install && yarn build && cd ..
cargo build --release
export env=dev
cp profiles/$env/config.yaml ./
tar zcvf app.tar.gz dist sea_file scripts config.yaml start.sh
```

## 镜像
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;这里为直接获得共享目录能力，使用了 samba alpine 镜像，如不需要该功能，转而去使用 scratch、alpine 也是一样的。

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;env、repo 根据实际情况填写：

```shell
cp target/release/sea_file .
strip sea_file && upx -9 sea_file
cp profiles/$env/* ./
tar zcvf app.tar.gz dist sea_file scripts config.yaml start.sh
mkdir -p docker && cp app.tar.gz .dockerfile docker/ && cd docker/
export repo=sea_file
docker build --pull -f .dockerfile --build-arg APP_ENV=$env -t $repo .
```

## 部署

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;因缺少 iOS、Android 等移动端 App，因此可以选择与 nextcloud 搭配使用。由于 nextcloud 的安装目录需要是空的，所以原理上需要先安装 nextcloud，再安装 SeaFile。同时，使用中需确保 SeaFile、NextCloud、smb 等全部服务的工作目录相同 。

- 从 nextcloud 上传的，只能在 nextcloud 浏览、使用。
- 从 smb、nfs、ftp、scp、nextcloud 等上传的，可在 SeaFile PC 页面中 检索、使用。

#### 安装 Nextcloud

```shell
docker run \
    -d -ti \
    --name nextcloud \
    -p 8000:80 \
    -v /data/nextcloud:/var/www/html \
    -v /data/samba:/var/www/html/data \
    nextcloud
```

#### 安装 SeaFile

```shell
# 1.部署 postgresql 数据库
docker run --restart=unless-stopped -p 5432:5432 -e POSTGRES_PASSWORD=postgres --name postgres -d postgres:12-alpine
# 2.部署 sea_file
docker run \
	-d -ti \
	--name sea_file \
	-p 8080:8080 -p 139:139 -p 445:445 \
	-v /data/samba:/mount \
	-e DATABASE_DSN=postgres://postgres:postgres@192.168.3.120:5432/postgres \
	abelco/sea_file \
	-u "user;123456" \
	-s "Samba;/mount/;yes;no;no;all;user;user"
```

## 挂载smb
### macOS
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Command + k，输入服务地址，并根据随后提示，输入账号、密码。
![img.png](docs/assets/macos-smb.jpg)
### Windows
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;右键，此电脑，添加一个网络位置，`\\192.168.3.120\share`。（或，开始，运行，输入地址）（结尾的 share，请根据实际情况输入）。
![img.png](docs/assets/win-smb.png)

## 迭代计划
| 序号 |                        功能                        | 进展 |
|:---:|:--------------------------------------------------:|:---:|
|  1  | 大范围的在线预览功能（txt、pdf、md、html、java、js、rs） | 开发中 |
|  2  |           在线文档功能（markdown、onlyoffice）        | ... |
|  3  |                  图片轮播功能（相册）                 | ... |
|  4  |                     视频播放功能                     | ... |
|  5  |                      多用户功能                      | ... |
|  6  |                单一来源的索引维护并行化                | ... |

- 来源：FS事件、巡视校准



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

#### chrome（待测试）
- --autoplay-policy=no-user-gesture-required
