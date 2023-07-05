FROM base-image as builder

FROM alpine
EXPOSE 8080/tcp

COPY --from=builder /federeddit-apps/users-service /usr/local/bin/users-service
COPY tools tools
CMD ["users-service", "--port", "8080", "--ip", "0.0.0.0"]