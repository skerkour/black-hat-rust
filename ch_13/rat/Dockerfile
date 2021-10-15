FROM ubuntu:latest

RUN apt update && apt -y upgrade
RUN apt -y install openssh-server

# authorize SSH connections to root account
RUN echo "PermitRootLogin yes" >> /etc/ssh/sshd_config
RUN echo "PasswordAuthentication yes" >> /etc/ssh/sshd_config
RUN service ssh restart

# change root password
RUN echo "root:root" | chpasswd

EXPOSE 22
CMD  ["/usr/sbin/sshd", "-D"]
