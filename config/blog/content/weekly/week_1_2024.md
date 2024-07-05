I started my first week of 2024 out strong.

# Projects

## ToDoKiosk
Recently, I have been using ToDoKiosk much more and it has become a crucial tool for personal organization.

Within this first week, I have made three releases of ToDoKiosk: 0.3.0 on January 2, 2024; 0.3.1 on January 5, 2024

## Website
I have released version 5.3.0 of the website along with content updates including this blog post.

This update introduces the "Services" page for showcasing what services I offer.

# Personal and Work

## Gigs
<figure>
    <img src="/static/services/jan52024_mc.webp">
    <figcaption><a href="../../projects/pc_pmma/">"Polymethylmethacrylate"</a> pictured on the left being prepared to test the server in-game - January 5, 2024</figcaption>
</figure>

On January 5, 2024, I helped troubleshoot and optimize a VPS-hosted Minecraft game server. This was my first ever paid tech support gig.

The initial problem with the server was apparent and was quickly resolved; three Minecraft instances were trying to be ran on this 1 vCPU/4GB RAM VPS, slowing down the system to the point where no one can join the Minecraft server and made SSH access slow.

Since I had root access to this VPS, I installed the lshw and htop utilities to further debug the system. I added a 4GB swapfile to match the RAM size as none was set up and the server appeared to have plenty of available storage (32GB allocated). I checked the CPU governor, which none was available since the system was virtualized. I checked the GRUB configuration while I researched if adding "mitigations=off" would have any improvement on AMD Zen 3 systems, since the host's CPU was an AMD EPYC 7543P, which adding this argument would have no benefit on this CPU. I did not add the "mitigations=off" argument and instead, I changed the GRUB bootloader timeout to 3 seconds from 5 seconds for a marginal difference in boot time.

In order to test server performance with multiple users, I joined the Minecraft server in order to see if there is too much server-side lag and other users are able to have operator command access.

The devices "Polymethylmethacrylate", being the HP ZBook Studio G3 15 (Xeon) and "Tetrahydrocannabinol", being the ASUS Zenfone 9 played a crucial role in the troubleshooting and setup of the server. The laptop, "Polymethylmethacrylate" was mainly used to access the Linux console via SSH and to test Minecraft functionality with the MultiMC launcher. The phone was used for hotspot connectivity to access the internet. Along with the CTCL systems, a provided Microsoft Surface device running Windows 11 was used to access the web interface of Hostinger and the VPS's hosted game interface.

This successful gig later lead to the addition of the "Services" page on this website.

