FROM base-image as builder
FROM alpine
EXPOSE 8080/tcp

COPY --from=builder /bin/users-service /usr/local/bin/users-service
CMD ["users-service", "--port", "8080", "--ip", "0.0.0.0"]