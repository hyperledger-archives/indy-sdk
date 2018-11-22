# Development 
FROM libindy
ARG uid=1000
RUN useradd -ms /bin/bash -u $uid node

WORKDIR vcx/wrappers/node
# Assumes we are in the ./vcx directory
RUN npm i npm@6.1.0
COPY vcx/libvcx/target/debian/*.deb .
RUN dpkg -i *.deb
USER node