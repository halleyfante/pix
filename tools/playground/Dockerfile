FROM rust:1.94 AS builder

WORKDIR /build
COPY compiler/Cargo.toml compiler/Cargo.lock ./
COPY compiler/source ./source

RUN cargo build --release

FROM node:22-slim

WORKDIR /app

COPY --from=builder /build/target/release/pix /usr/local/bin/pix
COPY --from=builder /build/target/release/pix-language-server /usr/local/bin/pix-language-server
COPY tools/playground/server.js ./server.js
COPY tools/playground/package.json ./frontend/package.json
COPY tools/playground/vite.config.ts ./frontend/vite.config.ts
COPY tools/playground/vitest.config.ts ./frontend/vitest.config.ts
COPY tools/playground/tsconfig.json ./frontend/tsconfig.json
COPY tools/playground/source ./frontend/source

RUN cd frontend && npm install && npx vite build && mv distribution ../public && cd .. && rm -rf frontend

EXPOSE 3000

CMD ["node", "server.js"]
