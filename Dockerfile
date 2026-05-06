# --- Stage 1: Build ---
FROM rust:1.84-slim as builder

# Системные зависимости
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    build-essential \
    && apt-get remove -y binaryen && apt-get autoremove -y \
    && rm -rf /var/lib/apt/lists/*

# Установка nightly и инструментов (Edition 2024 требует nightly в начале 2026 года)
RUN rustup toolchain install nightly && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown

# Установка Trunk
RUN cargo install --locked trunk

WORKDIR /app
COPY . .

# Сборка фронтенда
RUN trunk build --release

# --- Stage 2: Runtime ---
FROM nginx:alpine

# Копируем конфиг (ВАЖНО: создай этот файл рядом, см. ниже)
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Копируем артефакты сборки
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]