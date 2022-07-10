# 单人海量资料维查平台

![img.png](docs/assets/Logo.svg)

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;将文件拖入共享目录，之后便可在页面高效检索。服务监控文件系统，自动维护索引，索引基于DB。所说“海量”，要自备超大容量存储，像作者这里用的是ceph；并当检索效率不足时，自行另建高性能数据库。因此，理论上，只要中间件性能足够，就可支持无限量文件检索。

## 优势
- 使用原生共享目录，直接操作文件。
- 用户界面，立即可搜索。
- 检索的是DB，而非磁盘目录，性能更好。

 ## 环境
- CentOS、Alpine 等（ext4、xfs）
- macOS（APFS）
- Windows（待测试，理论上没问题）
- Internet（是的，界面服务使用了CDN资源，这意味着，作为发起请求的客户机，需要接入互联网，而非只需要内网。如确实希望剔除对公网的依赖，需要从源码自行编译，并移除CDN相关配置）

## 构建
```shell
cd webapp
yarn install && yarn build && cd ..
cargo build --release
```

## 镜像
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;这里为直接获得共享目录能力，使用了 samba alpine 镜像，如不需要该功能，转而去使用 scratch、alpine 也是一样的。

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;env、repo 根据实际情况填写：

```shell
cp target/release/SeaFile .
strip SeaFile && upx -9 SeaFile
cp profiles/$env/* ./
tar zcvf app.tar.gz favorites config.yaml
mkdir .docker && cp app.tar.gz .dockerfile .docker/ && cd .docker/
docker build --pull -f .dockerfile --build-arg APP_ENV=$env -t $repo .
```

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;因缺少 iOS、Android 等移动端App，当前，筛选并集成了 nextcloud，使用中需确保 SeaFile、NextCloud、smb 等全部服务的工作目录相同 。
- 从 nextcloud 中上传的，只能在 nextcloud 端浏览、使用。
- 从 smb、nfs、ftp、scp、nextcloud 等上传的，全部可在 SeaFile PC 页面中 检索、使用。 


## 挂载smb
### macOS
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Command + k，输入服务地址，并根据随后提示，输入账号、密码。
![img.png](docs/assets/macos-smb.jpg)
### Windows
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;右键，此电脑，添加一个网络位置，`\\192.168.3.120\share`。（或，开始，运行，输入地址）（结尾的 share，请根据实际情况输入）。
![img.png](docs/assets/win-smb.png)

## 迭代计划
|  序号   |                   功能                   | 进展  |
|:---:|:--------------------------------------:|:---:|
| 1| 大范围的在线预览功能（txt、pdf、md、html、java、js、rs） | 规划中 |
| 2|      在线文档功能（markdown、onlyoffice）       | ... |
| 3|               图片轮播功能（相册）             | ... |
| 4|                 视频播放功能                 | ... |
| 5|                 多用户功能                  | ... |




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
