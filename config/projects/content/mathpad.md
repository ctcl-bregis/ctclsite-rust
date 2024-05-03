# Introduction
MathPad was an idea to recreate HP's CalcPad device with modern components and design. The codename "Fort Myers" was chosen because the entire project was designed in Fort Myers, Florida during the November 2022 business trip for work at Fort Myers Beach, Florida. The device was developed with the idea of that it would be used with the Xactimate estimating software.

The project was the first of CTCL's circuit design projects to reach the assembly stage; PCB and components ordered and received.

The project was likely abandoned in December 2022 due to the lack of equipment and skills required to solder the tiny, delicate components. The project was officially canceled on January 19, 2024 after the introduction of an off-the-shelf product, the Logitech G600 MMO mouse into my workflow in January 2023. This project was made with the idea of necessity as I was expected to start having to use Xactimate after the conclusion of the business trip. The work I do with Xactimate requires almost no calculations as it is mostly reviewing existing drawings and measurements and creating a building sketch from that data.

On January 19, 2024, I released whatever work I had done for the project on [GitHub](https://github.com/ctcl-bregis/mathpad/tree/main). The existing components ordered may be reused for other projects.

In March 2024, I had the idea of trying to assemble a prototype as I now have more of the required equipment to place and solder the tiny components. The device would still have a use as I still find myself opening a software calculator for trivial calculations.

# Hardware
The device consists of a single PCB (circuit board) in the form of a keypad.

## PCB
The device uses a four-layer circuit board designed within EasyEDA. 

## Microcontroller
The device uses an STMicroelectronics STM32G0B1RCT6 ARM microcontroller. 

## Display
The device uses an OLED display for device status and showing math operations. The display chosen was the [Crystalfontz CFAL12864K-W 2.7" 128x64 OLED display module](https://www.crystalfontz.com/product/cfal12864kw-128x64-white-graphic-oled).

## Connectivity
The device features a standard USB Type-B. The reason to use Type-B over Type-C is unknown.

