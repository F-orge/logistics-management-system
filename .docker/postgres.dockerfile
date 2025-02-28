FROM postgres:17.2-alpine3.20 as base

FROM base as deps 

# Install necessary build steps

RUN apk add make && \
  apk add git

# Extensions

# PGJWT
RUN git clone https://github.com/michelp/pgjwt.git && \
  cd pgjwt && \ 
  make install