docker run \
    --name persistence-postgres \
    -d \
    -ePOSTGRES_USER=postgres \
    -ePOSTGRES_PASSWORD=postgres \
    -ePOSTGRES_HOST_AUTH_METHOD=trust \
    -p 5432:5432 \
    postgres:alpine || \
    docker start persistence-postgres
