# 单人海量资料平台

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;将文件拖入共享目录，之后便可在页面高效检索。服务监控文件系统事件，自动维护索引，索引基于DB实现。所说“海量”，一是要自备超大容量存储，像作者这里用的是ceph；二是当检索效率不足时，自行另建高性能数据库，以改善查询效率。理论上，只要中间件性能足够，就可支持无限量文件检索。

 ## 环境
- CentOS、Alpine 等（ext4、xfs）
- macOS（APFS）
- Windows（待测试）

## 构建
```shell
cargo build --release
```


## 未来
| 功能  |
|:---:|
| 多用户 |


### 参考资料
#### 页面
- https://github.com/actix/examples/tree/master/basics/static-files
- https://github.com/actix/examples/tree/master/basics/basics

#### 上传
- https://github.com/actix/examples/blob/master/forms/multipart-s3/src/utils/upload.rs
- https://github.com/actix/examples/blob/master/forms/multipart/src/main.rs

#### 下载
- https://actix.rs/docs/static-files/
- https://github.com/actix/examples/blob/master/basics/basics/src/main.rs
