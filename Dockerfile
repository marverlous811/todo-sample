FROM alpine:3.20 as build-state
ARG TARGETPLATFORM
ARG APPLICATION
WORKDIR /tmp
COPY ./release /tmp/release

RUN echo $TARGETPLATFORM
RUN echo $APPLICATION
RUN ls -R /tmp/

RUN case $TARGETPLATFORM in \
        "linux/amd64")  BUILD=x86_64-unknown-linux-gnu  ;; \
        "linux/arm64")  BUILD=aarch64-unknown-linux-gnu  ;; \
        *) exit 1 ;; \
    esac; \
    mv /tmp/release/$APPLICATION-$BUILD/realtime-server /application; \
    chmod +x /application
RUN /application --help

FROM alpine:3.20
WORKDIR /app
RUN apk add --no-cache libgcc
COPY --from=build-state /application /application
ENTRYPOINT ["/application"]