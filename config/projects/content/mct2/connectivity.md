
## Wired Connectivity

* 2x USB Type-C with PD and DisplayPort alternate mode
* 2x USB 3.2 Type-A
* 2x USB 2.0 Type-A
* HDMI
* 10/100/1000 Ethernet RJ-45

### Wireless Connectivity
Like most tablet computers, MediaCow Touch 2 has Wi-Fi and Bluetooth capabilities. 

Wi-Fi and Bluetooth is provided through a M.2 module. This is an important security and privacy feature as wireless connectivity can be disabled altogether by removing the module. A switch to shut off the wireless module is on the keypad.

### Ethernet
MediaCow Touch 2, like MediaCow Touch 1, has a single gigabit Ethernet port. 

#### Interface IC
The carrier board uses an Intel I210-series PCIe Ethernet transcevier IC. To my surprise, Intel's documentation on the chip is very thorough, with the I210 datasheet being 867 pages. 

I considered using yet another SiTime MEMS oscillator such as the SiT8008B for the 25MHz clock signal however, in contrast to SMEC, using an external clock source is more difficult than using a standard quartz crystal. The I210 would be paired with a standard quartz crystal with specifications outlined in the I210 datasheet.

## LattePanda Mu IO Usage
These are the connections from the LattePanda Mu to other devices, some may connect to external ports.

### HSIO Usage
MediaCow Touch 2, like many other devices using the LattePanda Mu, makes extensive use of the Flexible High Speed I/O (HSIO) feature of the Intel Processor N-series.

| Port    | Purpose    | PCIe Device | N100 PCIe Controller |
| ------- | ---------- | ----------- | -------------------- |
| HSIO 0  | USB 3.2    | N/A         | Controller #1        |
| HSIO 1  | USB 3.2    | N/A         | Controller #1        |
| HSIO 2  | M.2 Key E  | 1           | Controller #1        |
| HSIO 3  | Ethernet   | 2           | Controller #1        |
| HSIO 6  | **Unused** | N/A         | Controller #2        |
| HSIO 8  | M.2 Key M  | 3           | Controller #3        |
| HSIO 9  | M.2 Key M  | 3           | Controller #3        |
| HSIO 10 | M.2 Key M  | 3           | Controller #3        |
| HSIO 11 | M.2 Key M  | 3           | Controller #3        |

### Debug UART Header
A 2-pin 2.54mm standard pin header connected to UART0 is available on the carrier board. 

### SMBus and SMLink
Both the SMBus and SMLink interfaces are connected to SMEC. See the page on the [Embedded Controller](../ec/) for details.

### USB 2.0
All of the available USB 2.0 interfaces are used. 

| Port | Purpose                    | 
| ---- | -------------------------- |
| 1    | USB 3.0 Type A port 1      |
| 2    | USB 3.0 Type A port 2      |
| 3    | USB 2.0 Type A port 1      |
| 4    | USB 2.0 Type A port 2      |
| 5    | USB Type C port 1          |
| 6    | USB Type C port 2          |
| 7    | M.2 Key E Slot (Bluetooth) |
| 8    | Front-facing Camera        |

TCP0 and TCP1 ports must use either USB 2.0 port 1, 2, 5 or 6 as stated in the "Intel® Processor and Intel® Core™ i3 N-Series Datasheet, Volume 1 of 2" (Revision 001, January 2023) document.

### MIPI CSI
The FPC connector for MIPI CSI is used for the back-facing camera.

The MIPI CSI interfaces exposed by the connector, Port C and Port D, may be left unused.

### Embedded DisplayPort
The dedicated eDP connector on the LattePanda Mu is used to connect the touchscreen display to the system. A separate I2C connector is used for touch data. 

### TCP0/TCP1
Both TCP0/TCP1 are used for USB Type-C connectivity and have support for DisplayPort alternate mode. The DisplayPort alternate mode and USB 3.0 switching is implemented by the Intel N100 itself.

Notably the LattePanda Mu Full Evaluation Carrier Board uses one of the TCP interfaces for a secondary HDMI port. On MediaCow Touch 2, both are used for USB Type C. 

### TPM
The TPM header shares the same SPI bus as the external SPI flash for BIOS. 

The pinout of TPM headers seem to not follow any standard and the pinout varies by motherboard vendor. On MediaCow Touch 2, the pinout is identical to the TPM header pinout of the Supermicro H13SAE-MF motherboard.