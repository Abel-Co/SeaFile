#### Smb 负载均衡
- **Final =>** 
  1. 所有smb容器，启动后重建smb账号，之后 /*/min 扫描数据库，更新/创建 账号。
  2. 内网DNS解析域名到多个k8s-node-ip，已知 dnsmasq addn-hosts 支持单域名多ip 。
     1. 多ip时顺序是随机的，客户端一般使用第一个，以此近似LB，避免单点带宽压力。
     2. 当记录值有多个IP地址时，域名是如何解析的？ - https://support.huaweicloud.com/dns_faq/dns_faq_023.html
     3. 如果DNS返回多个IP，浏览器如何决定使用哪个？ - https://www.zhihu.com/question/605637765/answer/3066192097
- HaProxy:445
  ```shell
  # cat /etc/haproxy/haproxy.cfg
  global
      daemon
      maxconn 4096
  
  defaults
      mode tcp
      timeout connect 5s
      timeout client 50s
      timeout server 50s
      timeout check 3s
  
  listen 445
      bind *:445
      stick on src
      stick-table type ip size 200k expire 30m 
      server 172.17.16.110 172.17.16.110:446 check inter 1366 fall 2 rise 2 weight 1
  #    server 172.17.16.110 172.17.16.110:447 check inter 1366 fall 2 rise 2 weight 1
  ```
- **（成功）Smb多实例具有相同账号/密码 + HaProxy 445**

  - 变更haproxy, macOS Finder 不需要重新登录, 可持续浏览 smb 目录。

  ```shell
  docker run -d -it --name samba2 --restart unless-stopped -p 140:139 -p 446:445 -v /home/share:/mount dperson/samba -u "abel;123456" -s "share;/mount/;yes;no;no;all;abel;abel"
  docker run -d -it --name samba3 --restart unless-stopped -p 141:139 -p 447:445 -v /home/share:/mount dperson/samba -u "abel;123456" -s "share;/mount/;yes;no;no;all;abel;abel"
  ```
- **（失败）DNS负载均衡: 共享 /var/lib/samba/private/passdb.tdb 文件**

  - **得知：passdb.tdb 只是一份db文件，共享该文件，并不能将账号实际创建到各 smb容器实例的 linux 账户体系中去。**

  - （失败）auraco/samba:alpine 镜像 =>
  - 变更 haproxy, macOS Finder 不能持续浏览smb，samba3 未承认账号 abel，samba2 账号 abel 可用。

  ```shell
  docker rm -f samba2 samba3
  docker run -d -it --name samba2 --restart unless-stopped -p 140:139 -p 446:445 -v /home/share:/home/abel -v /var/lib/samba/:/var/lib/samba/ auraco/samba:alpine
  docker run -d -it --name samba3 --restart unless-stopped -p 141:139 -p 447:445 -v /home/share:/home/abel -v /var/lib/samba/:/var/lib/samba/ auraco/samba:alpine
  docker exec -ti samba2 sh
    "adduser -D abel
	  echo -e "123456\n123456\n" | smbpasswd -a -s abel
      exit"
  docker exec -ti samba2 sh -c rc-status ; docker exec -ti samba2 rc-service samba restart
  docker exec -ti samba3 sh -c rc-status ; docker exec -ti samba3 rc-service samba restart
  ```

  - （失败）dperson/samba 镜像 =>
  - 变更 haproxy, macOS Finder 不能持续浏览smb，user账号不可用，需要用user2账号。
  
  ```shell
  docker run -d -it --name samba2 -p 140:139 -p 446:445 -v /home/share:/mount -v /var/lib/samba/private/:/var/lib/samba/private/ dperson/samba -u "user;123456" -s "share;/mount/;yes;no;no;all;user;user"
  docker run -d -it --name samba3 -p 141:139 -p 447:445 -v /home/share:/mount -v /var/lib/samba/private/:/var/lib/samba/private/ dperson/samba -u "user2;123456" -s "share;/mount/;yes;no;no;all;user2;user2"
  ```

- Traefik tcp 代理
  - 由 traefik 做 tcp 445 代理。
  - 做会话保持、或 hash client ip 负载均衡 到 svc 。
- Ldap 管理用户
  - 先测试以域名挂载k8s下的seafile目录，若确实出现跨实例无法通信问题，以下为备选方案；
  - 已知 smb 支持 ldap 做账户管理。考虑为 seafile 内置 ldap-server，smb 以  127.0.0.1 使用自身的 ldap-server ；
    - [linux samba 配置ldap认证,Samba通过Openldap统一认证](https://blog.csdn.net/weixin_39616855/article/details/116963337)
  - Rust 实现的 ldap-server: lldap: [lldap/**lldap**](https://github.com/lldap/lldap) ，2.5k 星，53 Contributors 。
    - [samba4的负载均衡群集](https://blog.51cto.com/cmdschool/1829675)

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

#### 校正目录体积
- // 不能全表整体一批进行，因同一批，可能具有层级关系.
- // (若一定要考虑批量方案，则要为当前目录，统计'后代'全部'文件'，而不能是'子代'的'目录+文件')
- // ('子代'是通过parent_id串连，'后代'是通过path like /xx/% 串连，单条SQL最优耗时分别为：4ms、253ms)
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

#### 资源管理器打开网络位置
- https://www.google.com/search?q=%E7%94%A8%E8%B5%84%E6%BA%90%E7%AE%A1%E7%90%86%E5%99%A8%E6%89%93%E5%BC%80%E4%B8%80%E4%B8%AA%E7%9B%AE%E5%BD%95%E5%B9%B6%E8%87%AA%E5%8A%A8%E9%80%89%E5%AE%9A%E6%9F%90%E4%B8%AA%E6%96%87%E4%BB%B6&newwindow=1&sxsrf=AJOqlzUjVc1-wVBKHWzO942scJXve2XpAg%3A1675355949892&ei=LefbY9SHNr_j2roPl42P2AU&ved=0ahUKEwiU0o-io_f8AhW_sVYBHZfGA1sQ4dUDCA8&uact=5&oq=%E7%94%A8%E8%B5%84%E6%BA%90%E7%AE%A1%E7%90%86%E5%99%A8%E6%89%93%E5%BC%80%E4%B8%80%E4%B8%AA%E7%9B%AE%E5%BD%95%E5%B9%B6%E8%87%AA%E5%8A%A8%E9%80%89%E5%AE%9A%E6%9F%90%E4%B8%AA%E6%96%87%E4%BB%B6&gs_lcp=Cgxnd3Mtd2l6LXNlcnAQAzIKCAAQ8QQQHhCiBDoICAAQogQQsAM6DQgAEPEEEB4QogQQsANKBAhBGAFKBAhGGABQxARYxARgjQZoAXAAeACAAb8BiAG_AZIBAzAuMZgBAKABAqABAcgBBcABAQ&sclient=gws-wiz-serp
- https://jackxiang.com/post/7661/
- 

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