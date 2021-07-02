FROM mysql:5.7

ADD ./mysql.conf /etc/mysql/conf.d/my.cnf

RUN chmod 644 /etc/mysql/conf.d/my.cnf
RUN /etc/init.d/mysql restart