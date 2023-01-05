

```shell
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