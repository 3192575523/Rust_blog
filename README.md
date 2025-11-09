# Rust_blog_backend
A Rust blog backend built with Axum + Sqlite, the frontend is built with Vue, serving as an example skeleton
# Rust_blog_backend
A Rust blog backend built with Axum + Sqlite, the frontend is built with Vue, serving as an example skeleton

## Backend Operation
### env
Before the backend runs, you first need to configure the env environment. For local testing, use:
```
BIND=127.0.0.1:8080
DATABASE_URL=sqlite://blog.db
JWT_SECRET=use-a-long-random-secret
UPLOAD_DIR=./uploads
SITE_BASE=http://localhost:8079
```
### cargo run
! First, install sqlx-cli once:
```
cargo install sqlx-cli --no-default-features --features sqlite,rustls
```
! Create initial migration:
```
sqlx migrate add -r init
```
! Replace all SQL files with the existing SQL files
! Subsequently, implement the generation of the db file
```
sqlx database create
sqlx migrate run
```
! Compile and run
```
cargo clean
cargo run
```
# Vue_blog_fronted
```
npm clean
npm build
```
# Nginx
```

#user  nobody;
worker_processes  1;

#error_log  logs/error.log;
#error_log  logs/error.log  notice;
#error_log  logs/error.log  info;

#pid        logs/nginx.pid;


events {
    worker_connections  1024;
}


http {
    include       mime.types;
    default_type  application/octet-stream;

    #log_format  main  '$remote_addr - $remote_user [$time_local] "$request" '
    #                  '$status $body_bytes_sent "$http_referer" '
    #                  '"$http_user_agent" "$http_x_forwarded_for"';

    #access_log  logs/access.log  main;

    sendfile        on;
    #tcp_nopush     on;

    #keepalive_timeout  0;
    keepalive_timeout  65;

    #gzip  on;

    server {
        listen       8081;
        server_name  localhost;

        #access_log  logs/host.access.log  main;
        root   "D:\软件\Rust从0实现静态博客\blog-frontend_Using_test_V1\blog-frontend\dist";
        index  index.html;

        # SPA：除了实际存在的文件，其余都回退到 index.html
        location / {
            try_files $uri $uri/ /index.html;
        }

        # 对构建后的静态资源做长缓存
        location ~* \.(?:js|css|ico|gif|jpg|jpeg|png|svg|woff2?)$ {
            expires 30d;
            add_header Cache-Control "public, max-age=2592000, immutable";
            try_files $uri =404;
        }

        # index.html 不缓存，避免发布后看不到更新
        location = /index.html {
            add_header Cache-Control "no-store";
        }

        #error_page  404              /404.html;

        # 反向代理到 Axum 后端：API
        location /api/ {
            proxy_pass http://127.0.0.1:8080/api/;
            proxy_http_version 1.1;

            proxy_set_header Authorization $http_authorization;
            proxy_set_header Host              $host;
            proxy_set_header X-Real-IP         $remote_addr;
            proxy_set_header X-Forwarded-For   $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;

            proxy_connect_timeout 5s;
            proxy_read_timeout    60s;

            # 上传大小（按需调大）
            client_max_body_size 20m;
        }

        # 反代后端的静态上传目录
        location /uploads/ {
            proxy_pass http://127.0.0.1:8080/uploads/;
            proxy_set_header Host $host;
            proxy_read_timeout 60s;
        }

        # 简单压缩（可选）
        gzip on;
        gzip_min_length 1024;
        gzip_types text/plain text/css application/javascript application/json image/svg+xml;
    }
        # redirect server error pages to the static page /50x.html
        #
        # error_page   500 502 503 504  /50x.html;
        # location = /50x.html {
        #     root   html;
        # }

        # proxy the PHP scripts to Apache listening on 127.0.0.1:80
        #
        #location ~ \.php$ {
        #    proxy_pass   http://127.0.0.1;
        #}

        # pass the PHP scripts to FastCGI server listening on 127.0.0.1:9000
        #
        #location ~ \.php$ {
        #    root           html;
        #    fastcgi_pass   127.0.0.1:9000;
        #    fastcgi_index  index.php;
        #    fastcgi_param  SCRIPT_FILENAME  /scripts$fastcgi_script_name;
        #    include        fastcgi_params;
        #}

        # deny access to .htaccess files, if Apache's document root
        # concurs with nginx's one
        #
        #location ~ /\.ht {
        #    deny  all;
        #}
}


    # another virtual host using mix of IP-, name-, and port-based configuration
    #
    #server {
    #    listen       8000;
    #    listen       somename:8080;
    #    server_name  somename  alias  another.alias;

    #    location / {
    #        root   html;
    #        index  index.html index.htm;
    #    }
    #}


    # HTTPS server
    #
    #server {
    #    listen       443 ssl;
    #    server_name  localhost;

    #    ssl_certificate      cert.pem;
    #    ssl_certificate_key  cert.key;

    #    ssl_session_cache    shared:SSL:1m;
    #    ssl_session_timeout  5m;

    #    ssl_ciphers  HIGH:!aNULL:!MD5;
    #    ssl_prefer_server_ciphers  on;

    #    location / {
    #        root   html;
    #        index  index.html index.htm;
    #    }
    #}

```
