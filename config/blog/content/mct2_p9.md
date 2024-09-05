## Overview
Development on MediaCow Touch 2 has been slow since I have started the college semester and I have gained significant burnout from the project. This was my concern with not getting the project done by August 26, 2024. 

## Challenges

### Power Management

#### 12-20v to 3.3v 
A challenge that I have been facing is finding a switching buck converter that can step down the 12-20v VSYS voltage to 3.3v that is used by a variety of devices. Cutting costs is important especially due to the amount of these regulators expected to be used in the design. 

The main concerns were:
- Efficiency at low currents (1-200mA)
- Implementation cost
- Voltage input support

Up to August 30, 2024, I have had the plans to use Silergy SY21019 regulators extensively for most components that require 3.3v until I found out about the Diodes AP63203. On Mouser, as of August 30, 2024, the AP63203 is US$0.715 per unit with a quanity of 10 while the SY21019 is US$0.551/unit with a quanity of 10. I discovered that the implementation cost of the AP63203 would be less due to the regulator having a fixed output of 3.3v; does not require a resistor divider on the output to set the voltage. As result, the AP63203 would have a lower implementation cost while being more simple to use. 

The AP63203 seems to have good efficiency all around with the efficiency remaining above 70% from 1mA to 2A [according to the datasheet](https://www.diodes.com/assets/Datasheets/AP63200-AP63201-AP63203-AP63205.pdf). The SY21019 has similar performance with an inductor value of 22uH [as seen in its datasheet](https://us1.silergy.com/download/downloadFile?id=2877&type=product&ftype=note). The AP63203 uses a realitively small inductor value of around 3.3-3.9uH, I felt that I had to verify to this by using the formula in the datasheet.

After following the recommended layouts in the datasheets for both the AP63203 and SY21029. The AP63203 appears to use half the amount of external components.

An SY21019 may continue to be used for the 12v input to the OLED display on the side of the case. Another SY21019 is used to step VSYS down to 3.5v in a pre-regulator configuation for SMEC and sensors. 