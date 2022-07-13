FROM dperson/samba

WORKDIR /root

ADD app.tar.gz ./

ARG APP_ENV

ENV APP_ENV=$APP_ENV

ENTRYPOINT ["./sea_file"]

EXPOSE 8080