FROM mcr.microsoft.com/devcontainers/rust:bookworm

# ロケールの設定
RUN apt-get -y install locales && \
    localedef -f UTF-8 -i ja_JP ja_JP.UTF-8
ENV LANG ja_JP.UTF-8
ENV LANGUAGE ja_JP:ja
ENV LC_ALL ja_JP.UTF-8

# タイムゾーンの設定
RUN ln -sf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime && \
    echo "Asia/Tokyo" > /etc/timezone

# 作業ディレクトリを設定
WORKDIR /workspaces/app/