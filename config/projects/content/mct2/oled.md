## Button Pad
Due to difficulties of trying to fit many buttons on the side of the case while keeping the project to one PCB. I eventually decided to design a separate PCB that is basically a small keypad that would be mounted in the side of the case next to the OLED display. 

### Circuit Design
On August 1, 2024, the current design for the button board PCB was completed which makes use of the [TI TCA8418](https://www.ti.com/product/TCA8418) keypad scanner IC. 

Unlike example designs from TI with this IC, I added 1N4148 diodes on each button as one would commonly do when designing a mechanical computer keyboard. The diodes are there to prevent ghosting, which is mentioned in the datasheet for the TCA8418, where pressing multiple keys in a certain way would cause the IC to detect another key as pressed when it is not (the datasheet describes it better).

It is most likely that under normal operating conditions, ghosting would not be a concern but the diodes to prevent ghosting are there to prevent the possible circumstance that a certain key combination could have the keypad controller detect "Reset" or "Power" as being pressed when it is not and have the device shut off unexpectedly. 

### Mounting 
Currently, the PCB is designed with three mounting holes for use with M2-sized bolts. There is no fourth bolt, on the top left (when facing the button panel from outside of the case, right side up) as having the mounting post extend for both bolts on the left side would have it be in the way of the FPC connector.

### Functions
I eventually decided to have the power and reset buttons part of the keypad scan matrix. I avoided having the reset and power buttons part of a keypad or otherwise behind some sort of IO expander since I originally thought the IO expander or keypad controller would use too much power when the system is offline. However, the

## OLED Display
MediaCow Touch 2 would include a small OLED display on the side of the device next to the button panel. The idea to add an OLED display to MediaCow Touch 2 appeared on July 27, 2024 and I reluctantly decided to include it in the design.

Next to the button panel, there is an OLED display for showing various data about the device. 

### Display Selection
There was an OLED display I was going to use from Adafruit but it went out of stock. I instead looked at Crystalfontz, where I got the OLED display for MathPad.

After a while of comparing OLED displays from Crystalfontz, I eventually picked out the CFAL12832A-022W. 

### Interface
*See the [Embedded Controller](../ec/) page for details about how the display is interfaced*

The display is driven by the Embedded Controller; SMEC.
