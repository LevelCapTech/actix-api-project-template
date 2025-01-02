FROM mcr.microsoft.com/devcontainers/rust:bookworm

# ロケールの設定
RUN apt-get update
RUN apt-get -y install locales && \
    localedef -f UTF-8 -i ja_JP ja_JP.UTF-8
ENV LANG=ja_JP.UTF-8
ENV LANGUAGE=ja_JP:ja
ENV LC_ALL=ja_JP.UTF-8
    
# タイムゾーンの設定
RUN ln -sf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime && \
    echo "Asia/Tokyo" > /etc/timezone

# 環境変数設定 (npm グローバルパス)
ARG NPM_CONFIG_PREFIX=/home/node/.npm-global
ENV PATH=${NPM_CONFIG_PREFIX}/bin:${PATH}

# 必要なパッケージのインストール
RUN apt-get install -y openjdk-17-jre nodejs npm
RUN npm install -g @openapitools/openapi-generator-cli
RUN apt-get clean
RUN rm -rf /var/lib/apt/lists/*

# パーミッションの調整
RUN chown -R vscode:vscode /home/node/.npm-global

# 作業ディレクトリを設定
WORKDIR /workspaces/app/
