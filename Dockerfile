FROM eclipse-temurin:17-jre-alpine
RUN apk update && apk add bash
COPY ./ ./
RUN chmod +x lint
RUN wget "https://github.com/checkstyle/checkstyle/releases/download/checkstyle-8.39/checkstyle-8.39-all.jar" -O "checkstyle.jar"
ENTRYPOINT ["/bin/bash", "-l", "-c"]
