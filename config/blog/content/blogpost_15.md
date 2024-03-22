
# Projects

## Software
I have been learning more about Rust as I use it in projects.

### The Website
On Tuesday, March 19, 2024 I registered the domain ctcl.lgbt and it is now the current domain of the website. The former domain, ctcl-tech.com now redirects to ctcl.lgbt. The main benefit of this domain is that it is shorter than the former domain.

## Hardware Design
During this week, I have been exploring potential options for hardware design projects.

## PC Hardware
This week, I have started to finally do some more work with the servers.

### SVCS2 "Lisdexamfetamine"
On Monday, March 18, 2024, I have installed 64GB of Qimonda IMHH8GP22A1F2C-10F 8GB RDIMMs in SVCS2 "Lisdexamfetamine" as the 128GB configuration using 16GB SK hynix modules was unneeded. The Qimonda modules were presumably assembled in Dresden, Germany and likely the memory ICs were made there too. At this point, CTCL servers use just Nanya, Micron and Qimonda memory. Due to the memory modules making use of dual-die packages, Qimonda memory is the most utilized DDR3 vendor by die count

### SVCS3 "Dextroamphetamine"
At the beginning of the week, I have figured out why one of the HP ProLiant Blades, "Dextroamphetamine" did not boot from the RAID controller. It was simply the matter of setting the boot volume in the HP Option ROM Configuration of Arrays interface. This was prevalent since I was given the server in May 2023.

I installed Debian 12 on the system before trying to host [Jan](https://jan.ai) for use by other systems. This was not successful likely due to the CPUs' lack of the AVX2 extension. 

Soon after, I attempted to run Jan locally on [the desktop "Polybutylene Terephthalate"](../../projects/pc_pbt.md) with Linux Mint, this was successful on both the CPU and GPU. The system's AMD Radeon RX 6700 XT was able to be utilized with the experimental Vulkan support. Processing on the GPU was about six times faster than processing on the CPU. 

Later, I attempted to install Rocky Linux 9.3 on the server to introduce myself to the distribution as it is used extensively at VCU.
