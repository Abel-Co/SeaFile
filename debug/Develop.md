
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