FROM rust:bullseye

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ocl-icd-opencl-dev \
    opencl-headers \
    build-essential

WORKDIR /app

COPY Cargo.toml ./

RUN cargo fetch

COPY . .

CMD ["cargo", "run", "--release"]