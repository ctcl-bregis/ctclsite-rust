

## HSIO Usage
MediaCow Touch 2, like many other devices using the LattePanda Mu, makes extensive use of the Flexible High Speed I/O (HSIO) feature of the Intel Processor N-series.

| Port    | Purpose    | PCIe Device |
| ------- | ---------- | ----------- |
| HSIO 0  | USB 3.2    | N/A         |
| HSIO 1  | USB 3.2    | N/A         |
| HSIO 2  | M.2 Key E  | 1           |
| HSIO 3  | Ethernet   | 2           |
| HSIO 6  | **Unused** | N/A         |
| HSIO 8  | M.2 Key M  | 3           |
| HSIO 9  | M.2 Key M  | 3           |
| HSIO 10 | M.2 Key M  | 3           |
| HSIO 11 | M.2 Key M  | 3           |

## Wired Connectivity

* 2x USB Type-C with PD and DisplayPort alternate mode (see [Battery and Power Management](../power/) page for details on system power management)
* 2x USB 3.2 Type-A
* 2x USB 2.0 Type-A
* HDMI
* 10/100/1000 Ethernet RJ-45

### Wireless Connectivity
Like most tablet computers, MediaCow Touch 2 has Wi-Fi and Bluetooth capabilities. 

Wi-Fi and Bluetooth is provided through a M.2 module. This is an important security and privacy feature as wireless connectivity can be disabled altogether by removing the module. Along with modularity, there is a switch on the side of the device to shut off the module; a wireless kill switch.

### Ethernet
MediaCow Touch 2, like MediaCow Touch 1, has a single gigabit Ethernet port. 

#### Interface IC
The carrier board uses an Intel I210-series PCIe Ethernet transcevier IC. To my surprise, Intel's documentation on the chip is very thorough, with the I210 datasheet being 867 pages. 

I considered using yet another SiTime MEMS oscillator such as the SiT8008B for the 25MHz clock signal however, in contrast to SMEC, using an external clock source is more difficult than using a standard quartz crystal. The I210 would be paired with a standard quartz crystal with specifications outlined in the I210 datasheet.