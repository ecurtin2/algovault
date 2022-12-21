FROM python:3.11
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Predownload/compile deps
RUN cargo new algovault
WORKDIR /algovault
COPY Cargo.toml Cargo.lock /algovault/
RUN touch /algovault/src/lib.rs && cargo build


# Python dev stuff
COPY requirements_dev.txt .
RUN pip install -r requirements_dev.txt
ADD .pre-commit-config.yaml .
RUN pre-commit install


COPY . /algovault/
RUN cargo build
ENV PATH="/algovault/target/debug:${PATH}"
