#include  "main.h"

int wait_for_response(int target_port, int timeout_sec) {
    int sock = socket(AF_INET, SOCK_RAW, IPPROTO_TCP);
    if (sock < 0) return -1;

    struct timeval tv;
    tv.tv_sec = timeout_sec;
    tv.tv_usec = 0;
    setsockopt(sock, SOL_SOCKET, SO_RCVTIMEO, (const char*)&tv, sizeof(tv));

    char buffer[65536];
    while (1) {
        ssize_t data_len = recvfrom(sock, buffer, sizeof(buffer), 0, NULL, NULL);
        if (data_len < 0) {
            close(sock);
            return 0;
        }

        struct iphdr *iph = (struct iphdr *)buffer;
        struct tcphdr *tcph = (struct tcphdr *)(buffer + (iph->ihl * 4));

        if (ntohs(tcph->th_sport) == target_port) {
            close(sock);
            if ((tcph->th_flags & TH_SYN) && (tcph->th_flags & TH_ACK)) return 1;
            if (tcph->th_flags & TH_RST) return 2;
        }
    }
}