FROM auraco/samba:alpine

EXPOSE 8080 137/udp 138/udp 139 445

WORKDIR /root

ARG APP_ENV
ENV APP_ENV=$APP_ENV

ADD app.tar.gz ./

#ENTRYPOINT ["./seafile"]


#RUN apk add samba openrc \
#    && mkdir -p /run/openrc && touch /run/openrc/softlevel \
#    && chmod -R 711 /home/ && rc-update add samba # && rc-service samba start