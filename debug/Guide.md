#### Networking failed to start
- 最终补齐 /etc/network/interfaces 文件，从而解决。**以下可不填，文件留空即可**。
  ```ini
  auto lo
  iface lo inet loopback

  auto eth0
  iface eth0 inet static
      address 192.168.140.13/32
      gateway 172.26.16.2
      hostname alpine
  ```
- https://unix.stackexchange.com/questions/284791/networking-failed-to-start-on-alpine-linux

#### Smb 挂载目录轮询多实例，登录状态同步问题

- 先测试以域名挂载k8s下的seafile目录，若确实出现跨实例无法通信问题，以下为备选方案；
- 已知 smb 支持 ldap 做账户管理。考虑为 seafile 内置 ldap-server，smb 以  127.0.0.1 使用自身的 ldap-server ；
- Rust 实现的 ldap-server: lldap: [lldap/**lldap**](https://github.com/lldap/lldap) ，2.5k 星，53 Contributors 。

#### 校正目录体积
- // 不能全表整体一批进行，因同一批，可能具有层级关系.
- // (若一定要考虑批量方案，则要为当前目录，统计'后代'全部'文件'，而不能是'子代'的'目录+文件')
- // ('子代'是通过parent_id串连，'后代'是通过path like /xx/% 串连，单条最优耗时分别为：4ms、253ms)
- 发生登录时，校正一次用户级全量：
  ```sql
  select * from files where kind = 'Folder' and path like '/Users/Abel/%' order by length(path) desc;
  select coalesce(sum(size),0) from files where parent = 472308060426211328; -- {id}
  ```
- 每1.5分钟，加一条任务锁，向上统计一次，updated_at 在 1分钟内数据。 
- 分页：可以考虑使用 id > {id}，**但要确保'结果集'ID也是单调递增的**，因'雪花'主键是单调递增的，>{id}能尽可能搂取到新生成记录，避免遗漏.

#### 旭日图
- https://www.runoob.com/echarts/echarts-sunburst.html
  - https://www.runoob.com/try/try.php?filename=tryecharts_sunburst7
  - https://www.runoob.com/try/try.php?filename=tryecharts_sunburst6
  - https://echartsjs.com/
- https://blog.51cto.com/u_15477294/4892889
- https://blog.csdn.net/ansheng02/article/details/121869963

#### 图片转blob & base64
- https://blog.csdn.net/Benxiaohai_311/article/details/123230769
- https://blog.csdn.net/xuewenjie0217/article/details/123853375 

#### 系统平台检测
- https://cloud.tencent.com/developer/article/1538300
- navigator.userAgentData 仅适用于 Chrome 
- navigator.platform 已 @deprecated
- navigator.userAgent 将作为仅剩的
- 一个检测库：https://github.com/matthewhudson/current-device

#### 打开smb位置
- 浏览器
```html
<!-- macOS -->
smb://172.17.16.110/Yali
<!-- Arch -->
 
<!-- Windows -->
<!-- https://blog.csdn.net/Jinzhenjie/article/details/105066681 -->
<!-- https://blog.csdn.net/Jinzhenjie/article/details/105045064 -->
<!-- https://juejin.cn/post/6979391468309839879 -->
smb:\\172.17.16.110\Yali
```

- Windows

  需导入以下注册表信息。（保存为任意名称 .reg，双击导入）

```ini
Windows Registry Editor Version 5.00

[HKEY_CLASSES_ROOT\smb]
@="URL: smb protocol"
"URL Protocol"=""

[HKEY_CLASSES_ROOT\smb\shell]

[HKEY_CLASSES_ROOT\smb\shell\open]

[HKEY_CLASSES_ROOT\smb\shell\open\command]
@="cmd /c start \\\\172.17.16.110\\Yali"
```
> @="cmd /c start \\\\172.17.16.110\\Yali" （会弹 cmd 窗口，且不自动消失）
> VBS 输出日志：https://blog.csdn.net/weixin_34376562/article/details/91642691

- 命令行
```shell
# macOS
open smb://172.17.16.110/Yali
# Arch
dolphin smb://172.17.16.110/Yali
# Windows
## 挂载：net use Z: \\172.17.16.110\Yali
## 1.先认证
start \\172.17.16.110\Yali
## 2.浏览
explorer \\172.17.16.110\Yali
# Ubuntu
nautilus smb://172.17.16.110/Yali
```
> [Windows系统挂载SMB文件系统](https://help.aliyun.com/document_detail/171332.html)

#### Run Docker

```shell
# test seafile image
# --pull always => docker pull registry.cn-beijing.aliyuncs.com/wcik/seafile:dev01
docker run -d -ti --pull always --name seafile --restart unless-stopped -p 8080:8080 -p 139:139 -p 445:445 -v /data/samba:/home registry.cn-beijing.aliyuncs.com/wcik/seafile:dev01
```
> docker run --rm -ti --name seafile --entrypoint sh -p 8080:8080 -p 139:139 -p 445:445 -v /data/samba:/home registry.cn-beijing.aliyuncs.com/wcik/seafile:dev01

```shell
# debug smb account
docker build -f rust-smb.dockerfile -t rust-smb .
docker run --rm -ti --name seafile --entrypoint zsh -p 8080:8080 -p 139:139 -p 445:445 -v /root/work:/root rust-smb
```
> open smb://172.17.16.110/xugy

```shell
docker run -d -ti --name samba --restart unless-stopped -p 139:139 -p 445:445 -v /data/samba:/home alpine
```


------------------------------------------------------------------------------------------------------------------------

```shell
rc-status | grep samba
rc-service samba restart
```

```shell
adduser -D Yali
echo -e "123456\n123456\n" | smbpasswd -a -s yali
```
> `deluser --remove-home Xugy group`
> 