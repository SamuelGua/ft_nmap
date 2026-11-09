#include  "main.h"

int send_custom_tcp(const char *ip_str, int dport, int sport, uint32_t seq, uint8_t flags) {
    int sock = socket(AF_INET, SOCK_RAW, IPPROTO_TCP);
    if (sock < 0) return -1;

    struct sockaddr_in target;
    memset(&target, 0, sizeof(target));
    target.sin_family = AF_INET;
    target.sin_addr.s_addr = inet_addr(ip_str);
    target.sin_port = htons(dport);

    char packet[40];
    memset(packet, 0, 40);

    struct tcphdr *tcph = (struct tcphdr *)packet;
    tcph->th_sport = htons(sport);
    tcph->th_dport = htons(dport);
    tcph->th_seq = htonl(seq);
    tcph->th_flags = flags;
    tcph->th_win = htons(65535);
    tcph->th_sum = 0;

    int sent = sendto(sock, packet, sizeof(packet), 0, (struct sockaddr *)&target, sizeof(target));
    close(sock);
    return (sent < 0) ? -1 : 0;
}
