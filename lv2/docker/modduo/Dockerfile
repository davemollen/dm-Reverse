FROM ubuntu:16.04 as modduo-build

# Set root user
USER root

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl \ 
    gcc-4.9-arm-linux-gnueabihf \
    libc6-dev-i386
    
# Get LLVM
RUN curl -SL https://releases.llvm.org/7.0.0/clang+llvm-7.0.0-x86_64-linux-gnu-ubuntu-16.04.tar.xz \
 | tar -xJC . && \
 mv clang+llvm-7.0.0-x86_64-linux-gnu-ubuntu-16.04 llvm
ENV PATH="/llvm/bin:$PATH"
ENV LLVM_PATH="/llvm:$LLVM_PATH"
ENV LD_LIBRARY_PATH="/llvm/lib:$LD_LIBRARY_PATH"
ENV DYLD_LIBRARY_PATH="/llvm/lib:$DYLD_LIBRARY_PATH"

# Get Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add armv7-unknown-linux-gnueabihf

# Copy files
COPY ./ ./

# Build 
RUN cd lv2 && cargo build --release --target armv7-unknown-linux-gnueabihf

# Export
FROM scratch AS modduo-export
COPY --from=modduo-build lv2/target/armv7-unknown-linux-gnueabihf/release/libdm_reverse.so .