FROM ubuntu:18.04


# Setup gpg key and deb repo
RUN apt-get update && apt-get install -y curl gnupg && \
    curl -sSL http://deb.openalpr.com/openalpr.gpg.key | apt-key add - && \
    echo "deb http://deb.openalpr.com/bionic/ bionic main" > /etc/apt/sources.list.d/openalpr.list && \
	apt-get update && apt-get install -y \
      libalprstream-dev \
      openalpr \
      openalpr-daemon \
      openalpr-link \
      python-alprstream \
      python-openalpr \
      supervisor && \
      touch /usr/share/openalpr/is_docker && \
      rm -rf /var/lib/apt/lists/*



# Expose the beanstalkd port and the web service
VOLUME /etc/openalpr/
RUN mkdir -p   /var/lib/openalpr/plateimages
VOLUME /var/lib/openalpr/plateimages
RUN chown 1000:1000 /var/lib/openalpr/plateimages

RUN mkdir -p   /var/log/supervisor
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf
COPY alprd.conf /etc/openalpr/alprd.conf
CMD ["/usr/bin/supervisord"]
WORKDIR ~/