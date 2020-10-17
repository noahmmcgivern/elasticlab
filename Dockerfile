#
#

# BUILD

FROM debian:bullseye-slim

MAINTAINER noahmmcgivern

WORKDIR /root/

COPY bin/ bin/
COPY provision/ provision/
RUN chmod 755 -R provision/

RUN apt-get update
RUN ./provision/dependencies.sh

RUN apt-get install -y curl
RUN ./provision/rust.sh
RUN . $HOME/.cargo/env && \
cd bin && \
cargo b --release

#
#

# RUN

FROM debian:bullseye-slim

COPY --from=0 /root/bin/target/release/elasticlab /usr/local/bin/el
COPY infra/ /root/.elasticlab/infra/

RUN apt-get update
RUN apt-get install -y curl unzip 
RUN curl -s -L -o "terraform.zip" "https://releases.hashicorp.com/terraform/0.13.4/terraform_0.13.4_linux_amd64.zip"
RUN unzip "terraform.zip" && rm "terraform.zip"
RUN mv "terraform" "/usr/local/bin/"
RUN apt-get purge -y curl unzip
RUN apt-get autoremove -y
