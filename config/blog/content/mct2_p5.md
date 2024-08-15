

## Challenges

### Button Board and OLED
On August 3, 2024, I had to redesign the Button Board PCB as I misjudged its orientation in the device. Having to redesign the board, however, allowed me to clean up the design and, this time, the design process somehow went much faster. The Texas Instruments TCA8418 keypad controller IC is still used. 

The OLED display chosen for the project is the Crystalfontz [CFAL12832A-022W](https://www.crystalfontz.com/product/cfal12832a022w-128x32-small-oled-display) 128x32 OLED display.

I have started to implement the OLED display into the schematic. Instead of using SPI or I2C for the display, I decided to use the parallel Intel 8080 interface to communicate with the display as this would have the display act as a memory-mapped device on the STM32 using the [FMC (Flexible Memory Controller)](https://www.st.com/resource/en/product_training/STM32F7_Memory_FSMC.pdf) [PDF] feature.

### USB Power Delivery
USB Power Delivery remains one of the most difficult and annoying parts of the design. I had to switch from the TPS65994AD to the TPS65988 because the former is apparently meant for Thunderbolt applications.

I also switched the module SiSF00DN for BSZ086P03NS3E since the TPS65988 uses P-type MOSFETs. This allows me to use the Infineon TSDSON-8 footprint I "accidently" drew since the BSZ086P03NS3E uses that package type.

### Battery Management
As of August 4, 2024, I still do not have an HP FM08-compatible battery pack. I still have a little bit of doubt despite it being physically possible. The doubt is likely due to how I am unsure what the pinout actually is, the size of the battery pack being unknown, the target per-cell charge voltage being unknown and what kind of connector it has being unknown. I could figure all of it by getting the battery pack which I do know for sure would be suitable as it is indeed a 4S2P (14.4V) battery pack.

My best bet is to just purchase one of these battery packs as I could get information on size, pinout and connector type though there is yet another concern being where to get the battery pack from. I might as well take the risk and get the battery pack off from a known-good source on Amazon. Even if I had the battery pack, it would likely still be difficult for me to know what charge voltage to use.

### Power Management
Power management remains to be the most difficult part of the project, with the painfully complicated-looking schematic of the USB Type-C ports and battery charger IC that I have so far.

The plan to use the BTS7006-1EPP for switching power to the Mu and other components still remains.

### Embedded Controller
SMEC will be one of the most complex parts of the project though likely the most enjoyable part. 

Due to the lack of support for the STM32U5 series by the [stm32-rs](https://github.com/stm32-rs/stm32-rs) project, I decided to go back to using an STM32L4-series microcontroller. The microcontroller that is now used in the project is the STM32L496VGT6. Though there is a somewhat small chance I would not use Rust for writing the firmware for SMEC, I'd rather ensure that the microcontroller would have support for Rust.

## Social Media Activity
On August 1, 2024, the Instagram account for LattePanda (lattepanda_cn) [posted two of my photos of test setups of the LattePanda Mu Lite Carrier Board](https://www.instagram.com/p/C-ILIotJvlu/).

The first photo shows the Windows 11 setup where the Lite Carrier Board with the Mu is shown on top of the WBPC2 "Polycarbonate" desktop (HP Z240). The second photo is of me testing the eDP display connection but not the photo of the display in action that shows the Linux Mint neofetch readout. The images were likely pulled from this website. 

I did not have any prior notice about the Instagram posts though these pictures are freely available. It is likely that the team did not know about my presence on Instagram.

The description does seem to have some influence from blog posts that I have written about the project so far. 
