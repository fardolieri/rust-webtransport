openssl req -new -x509 -nodes \
    -days 14 \
    -newkey ec:<(openssl ecparam -name prime256v1) \
    -keyout localhost.key \
    -out localhost.crt \
    -subj "/CN=localhost" \
    -addext "subjectAltName = DNS:localhost,IP:127.0.0.1"
