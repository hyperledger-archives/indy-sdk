# Development 
FROM libvcx
 
# Update environment 
# JRE installation and gcc 
RUN apt-get update -y && apt-get install -y ca-certificates \ 
    debhelper \ 
    curl  
 
# Install Nodejs  
RUN curl -sL https://deb.nodesource.com/setup_8.x | bash - \ 
    && apt-get install -y nodejs 

# Assumes we are in the ./vcx directory
RUN npm i -g npm@6.1.0

WORKDIR /sdk/vcx/wrappers/node

CMD [ "/sdk/vcx/wrappers/node/ci/test-and-package.sh" ]


