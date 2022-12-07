FROM alpine

RUN apk add samba openrc --no-cache \
    && mkdir -p /run/openrc && touch /run/openrc/softlevel \
    && chmod -R 711 /home/ && rc-update add samba # && rc-service samba start

WORKDIR /root

ADD app.tar.gz ./

ARG APP_ENV

ENV APP_ENV=$APP_ENV

EXPOSE 8080 137/udp 138/udp 139 445

#ENTRYPOINT ["./seafile"]
