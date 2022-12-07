





```shell
docker run -d -ti --name samba --restart unless-stopped -p 139:139 -p 445:445 -v /data/samba:/home alpine
```

```shell
rc-status
rc-service samba start
```

```shell
adduser -D yali
echo -e "123456\n123456\n" | smbpasswd -a -s yali
```