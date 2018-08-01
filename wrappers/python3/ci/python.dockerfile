# Development
FROM libvcx

RUN apt-get update && apt-get install -y python3

RUN apt-get install -y python3-pip

RUN pip3 install pytest==3.4.2 qrcode pytest-asyncio

ENV PYTHONPATH=${PYTHONPATH}:/sdk/vcx/wrappers/python3

WORKDIR /sdk/vcx/wrappers/python3/tests

RUN find . -name \*.pyc -delete

# for testing, remove for production
COPY vcx/wrappers/python3/ci/test-and-package.sh /sdk/vcx/wrappers/python3/ci/

CMD [ "/sdk/vcx/wrappers/python3/ci/test-and-package.sh" ]

