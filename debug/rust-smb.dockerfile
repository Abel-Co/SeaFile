#FROM rust:alpine
FROM auraco/rust:alpine

ENV TZ=Asia/Shanghai

RUN echo https://mirrors.aliyun.com/alpine/latest-stable/main > /etc/apk/repositories \
    && echo https://mirrors.aliyun.com/alpine/latest-stable/community >> /etc/apk/repositories \
    && apk add musl-dev openssl-dev upx samba openrc \
        && mkdir -p /run/openrc && touch /run/openrc/softlevel \
        && chmod -R 711 /home/ && rc-update add samba

RUN apk add zsh git curl tree vim tzdata \
    && sed -i 's@root:x:0:0:root:/root:/bin/ash@root:x:0:0:root:/root:/bin/zsh@g' /etc/passwd

RUN git clone https://gitee.com/mirrors/oh-my-zsh.git /root/.oh-my-zsh \
        && cp /root/.oh-my-zsh/templates/zshrc.zsh-template /root/.zshrc \
        && git clone https://gitee.com/dawnwords/zsh-syntax-highlighting.git /root/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting \
        && sed -i 's/ZSH_THEME="robbyrussell"/ZSH_THEME="garyblessington"/g' /root/.zshrc \
        && sed -i 's/plugins=(git)/plugins=(git zsh-syntax-highlighting)/g' /root/.zshrc \
        && echo "alias 340='sync && echo 3 > /proc/sys/vm/drop_caches'" >> /root/.zshrc \
        && echo 'function f() { find . -iname "*$1*" ${@:2} }' >> /root/.zshrc \
        && echo 'function r() { grep "$1" ${@:2} -R . }' >> /root/.zshrc \
        && echo 'function mkcd() { mkdir -p "$@" && cd "$_"; }' >> /root/.zshrc

EXPOSE 8080 137/udp 138/udp 139 445

WORKDIR /root