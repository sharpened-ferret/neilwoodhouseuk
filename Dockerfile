FROM nginx:bookworm

WORKDIR /APP

COPY ./target/release/neilwoodhouseuk bin/
COPY ./neilwoodhouse.uk.conf /etc/nginx/conf.d/default.conf
COPY ./entrypoint.sh ./

CMD ["/bin/sh", "/APP/entrypoint.sh"]