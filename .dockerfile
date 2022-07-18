FROM dperson/samba

WORKDIR /root

ADD app.tar.gz ./

ARG APP_ENV

ENV APP_ENV=$APP_ENV

EXPOSE 8080 137/udp 138/udp 139 445

ENTRYPOINT ["start.sh"]
