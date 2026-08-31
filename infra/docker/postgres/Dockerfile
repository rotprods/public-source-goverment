FROM postgis/postgis:18-3.6

ARG PGVECTOR_VERSION=v0.8.6

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       ca-certificates \
       build-essential \
       git \
       postgresql-server-dev-18 \
    && git clone --depth 1 --branch "${PGVECTOR_VERSION}" https://github.com/pgvector/pgvector.git /tmp/pgvector \
    && make -C /tmp/pgvector \
    && make -C /tmp/pgvector install \
    && rm -rf /tmp/pgvector \
    && apt-get purge -y --auto-remove build-essential git postgresql-server-dev-18 \
    && rm -rf /var/lib/apt/lists/*

COPY infra/docker/postgres/init/001-extensions.sql /docker-entrypoint-initdb.d/001-extensions.sql
