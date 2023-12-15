FROM alpine:latest as copy

RUN mkdir ./bins

COPY httpget.* ./bins/

RUN ls ./bins && \
    mv "./bins/httpget.$(arch)" ./bins/httpget && \
    mv "./bins/httpget-tls.$(arch)" ./bins/httpget-tls

FROM scratch as runner

COPY --from=copy /bins/httpget /

ENTRYPOINT ["/httpget"]

FROM scratch as runner-tls

COPY --from=copy /bins/httpget-tls /httpget

ENTRYPOINT ["/httpget"]