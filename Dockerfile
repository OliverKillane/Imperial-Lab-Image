# Note the from image is deprecated 
FROM openjdk:17-alpine
RUN apk update && apk add bash
COPY ./ ./
RUN chmod +x lint
ENV PATH="${PATH}:/"
RUN wget "https://github.com/checkstyle/checkstyle/releases/download/checkstyle-8.39/checkstyle-8.39-all.jar" -O "checkstyle.jar"
ENTRYPOINT ["/bin/bash", "-l", "-c"]
