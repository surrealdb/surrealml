FROM rust:latest

WORKDIR /app
SHELL ["/bin/bash", "-c"]

RUN apt-get update
RUN apt-get install libclang-dev -y
RUN apt-get install python3.11-dev -y
RUN apt-get install python3-pip -y
RUN apt-get install build-essential -y
RUN apt-get install python3-venv -y
RUN bash

# RUN python3 -m venv venv
# RUN source venv/bin/activate && pip install torch==2.0.0

COPY . /app/
# RUN cp -r /app/venv/lib/python3.11/site-packages/torch /app/surrealdb

# ENV LIBTORCH_USE_PYTORCH="1"
# ENV LIBTORCH_STATIC="1"
# ENV LIBTORCH_CXX11_ABI="0"
# ENV LIBTORCH=/app/venv/lib/python3.11/site-packages/torch
# ENV DYLD_LIBRARY_PATH=/app/venv/lib/python3.11/site-packages/torch/lib
# ENV LD_LIBRARY_PATH=/app/venv/lib/python3.11/site-packages/torch/lib

# RUN source venv/bin/activate && cd surrealdb && cargo build
# RUN cargo build

EXPOSE 8000
RUN cargo build

# CMD ["/bin/bash", "build_script.sh", "RUN"]
CMD ["tail", "-f", "/dev/null"]
# CMD ["/bin/bash", "cargo run -- start --log trace --user root --pass root memory"]
