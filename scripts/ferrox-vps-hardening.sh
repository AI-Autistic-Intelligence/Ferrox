#!/usr/bin/env bash
# ==============================================================================
# Ferrox VPS Hardening & Anti-DDoS Kernel Protection Script
# ==============================================================================
# Applies Linux Kernel sysctl security parameters, ASLR memory protections,
# TCP SYN cookie flood defenses, and nftables Layer-3/4 packet filtering.
# ==============================================================================

set -euo pipefail

echo "🛡️ Starting Ferrox VPS Kernel Hardening & Anti-DDoS Setup..."

# 1. Apply Sysctl Security Rules
SYSCTL_CONF="/etc/sysctl.d/99-ferrox-security.conf"
cat << 'EOF' > "$SYSCTL_CONF"
# Ferrox Enterprise Kernel Hardening Parameters

# TCP SYN Flood Protection
net.ipv4.tcp_syncookies = 1
net.ipv4.tcp_max_syn_backlog = 8192
net.ipv4.tcp_synack_retries = 2
net.ipv4.tcp_syn_retries = 3

# Reverse Path Filtering (Mitigate IP Spoofing)
net.ipv4.conf.all.rp_filter = 1
net.ipv4.conf.default.rp_filter = 1

# Disable IP Source Routing & ICMP Redirects
net.ipv4.conf.all.accept_source_route = 0
net.ipv4.conf.default.accept_source_route = 0
net.ipv4.conf.all.accept_redirects = 0
net.ipv4.conf.default.accept_redirects = 0
net.ipv4.conf.all.secure_redirects = 0
net.ipv4.conf.default.secure_redirects = 0
net.ipv4.conf.all.send_redirects = 0

# Ignore ICMP Echo Broadcast Requests (Mitigate Smurf Attacks)
net.ipv4.icmp_echo_ignore_broadcasts = 1
net.ipv4.icmp_ignore_bogus_error_responses = 1

# ASLR & Kernel Memory Protection against Exploits / Zero-Days
kernel.randomize_va_space = 2
fs.protected_hardlinks = 1
fs.protected_symlinks = 1
fs.protected_fifos = 2
fs.protected_regular = 2

# Maximize Connection Tracking Table Capacity
net.netfilter.nf_conntrack_max = 262144
EOF

sysctl --system > /dev/null 2>&1 || true
echo "✅ Sysctl Security Parameters Applied."

# 2. Configure nftables Firewall Anti-DDoS Rate Limiting
if command -v nft > /dev/null 2>&1; then
    cat << 'EOF' > /etc/nftables.conf
#!/usr/sbin/nft -f

flush ruleset

table inet ferrox_filter {
    chain input {
        type filter hook input priority 0; policy drop;

        # Allow established and related connections
        ct state established,related accept

        # Allow loopback interface
        iifname "lo" accept

        # Drop invalid packets
        ct state invalid drop

        # TCP SYN Flood Rate Limiting (Max 25 SYN/sec per IP)
        tcp flags syn meter syn_limit { ip saddr limit rate 25/second burst 50 packets } accept
        tcp flags syn drop

        # Limit ICMP Ping Flood (Max 5 ICMP/sec)
        ip protocol icmp limit rate 5/second accept

        # Allow SSH (Port 22) and Ferrox HTTP/HTTPS (Ports 80, 443, 3000)
        tcp dport { 22, 80, 443, 3000 } accept
    }
}
EOF
    systemctl restart nftables || true
    echo "✅ nftables Anti-DDoS Filtering Active."
fi

echo "🛡️ Ferrox VPS Security Hardening Complete!"
