



```shell
# Debug
docker run --rm -ti --name seafile --entrypoint sh -p 8080:8080 -p 139:139 -p 445:445 registry.cn-beijing.aliyuncs.com/wcik/seafile:dev01
```
> `docker run --rm -ti --name seafile --entrypoint sh -p 8080:8080 -p 139:139 -p 445:445 -v /data/samba:/home registry.cn-beijing.aliyuncs.com/wcik/seafile:dev01`
> open smb://172.17.16.110/xugy

```shell
docker run -d -ti --name samba --restart unless-stopped -p 139:139 -p 445:445 -v /data/samba:/home alpine
```

```shell
rc-status
rc-service samba start
```

```shell
adduser -D Yali
echo -e "123456\n123456\n" | smbpasswd -a -s yali
```
> `deluser --remove-home Xugy groupdeluser --remove-home Yali group`
> 