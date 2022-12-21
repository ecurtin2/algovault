FROM python:3.11-slim
RUN apt update && apt install git curl build-essential -y
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /bin

# Predownload/compile deps
RUN cargo new algovault
WORKDIR /algovault
COPY Cargo.toml Cargo.lock /algovault/
RUN touch /algovault/src/lib.rs && cargo build

# Python dev stuff
COPY requirements_dev.txt .
RUN pip install -r requirements_dev.txt


COPY . /algovault/
RUN just build
ENV PATH="/algovault/target/debug:${PATH}"
