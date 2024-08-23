
## Project Status Update
Instead of the usual layout that I would have here, I have more of an announcement to make.

On August 22, 2024, I got a response from the LattePanda team about what is required for the deadline. Somehow I accidently sent the email from an iCloud account since it defaulted to that on the iOS Mail app.

The response, with names omitted, was:
```
[...]
After the project is completed, please share it on the LattePanda Forum with a project overview, BOM list, schematic diagram, PCB (hyperlinked to Github), photos, and optionally a video.
[...]
```

I am not the only one who had the difficulty of completing the project within the time period given so it is likely that we may get an extension though if it does happen, I **must** complete the project. 

### What Happened
Multiple factors got in the way of progress.

#### Skill Overestimation
This is not the first time I have overestimated my skillset by having myself develop this exact kind of project within a small time period. I have done this likely two or more times in my senior (2021-2022) year of high school. Back then, I expected to complete the project including a physical prototype within a two-week period instead of the two months I had for this attempt. Failing to meet these deadlines time and time again had me discouraged in electronics engineering and I eventually decided to just work on software.

Since I had two months to complete the project, I thought I would be able to get it done.

As mentioned several times before: I did not expect to have the project to be accepted into the LattePanda Mu Free Trial Event.

#### Specifics
For the first half of July, I had personal difficulties that had me unable to complete any meaningful amount of work on the project. 

Throughout most of the time I had, I was suffering from the effects of social isolation which has historically done a massive toll on my productivity, especially since the beginning of the 2020s.

In August 2024, I had *three* instances where I had to travel. On the third, which starts today, August 22, 2024, I may not have a reliable Internet connection for the duration of the trip. I also told myself to enjoy this trip; actually have it a vacation instead of working the entire time like I just did during the trip to Atlantic Beach, North Carolina. 

## Challenges
These are the challenges faced in the project currently.

### Power Management
Maintaining good power efficiency with the many DC-DC converters while keeping costs down has been a major challenge mostly due to the such high voltage range from the battery pack and USB PD subsystem which is expected to be anywhere between 12 to 21 volts. 

LDO (Linear) regulators' efficiency is proportional to the difference in voltage from the input to the output. The efficiency is equal to the output voltage divided by the input voltage. For example, the efficiency of an LDO circuit that steps 21 volts down to 3.3 volts is 15.71%. If there is a device that uses 100mA at 3.3v and a linear regulator is used to step down that voltage from 21v, the input of the linear regulator would be 100mA at 21v. As result, the regulator would dissipate *1.77 watts* of power as heat. 

As result, no LDOs are to be used on the system supply (VSYS/VSB) unless they are part of an IC such as the LDO_3V3 found in the USB PD controller. 

### USB Power Delivery
At this point, I may have figured out the implementation of USB Power Delivery.

### Sensors
A last minute feature I added was the addition of a Gyroscope/Accelerometer/Magnetometer module and Barometric Pressure Sensor module. The parts chosen were the TDK InvenSense ICM-20948 and Bosch Sensortec BMP384, respectively. 

MediaCow Touch 1 lacked a Gyroscope/Accelerometer which was of some annoyance due to Android's reliance on such data for features such as automatic screen rotation. I did not really consider adding such sensors until the last minute since I did not know how to have the OS make use of the data.

These two sensor modules may be connected to the embedded controller (SMEC) where the data would be made available to the operating system by the embedded controller. This allows for SMEC to display readouts of the sensors' data on the OLED display independent of the operating system.

### PCIe
I have been having some confusion on the placement of AC decoupling capacitors for the PCIe differential pairs. The specific issue is if they are needed for PCIe traces for transmitting data to the Mu; if the LattePanda Mu contains such capacitors on the RX pins of the Intel N100 SoC. 

The capacitors on the N100 PCIe RX pins are not present on the schematic for the Full Evaluation Carrier Board. An idea I had was to just put 0-ohm resistors (jumpers) where the AC coupling capacitors would be on the RX pins on the module connector. 

### Embedded Controller (SMEC)
I strangely did not notice I had the wrong microcontroller on the schematic until August 21, 2024. However, switching the microcontroller symbol for the correct part was trivial.

### Cooling Fan
Currently, the plan is to have a blower fan that pulls air from the back side of the device and create airflow over the M.2 SSD and LattePanda Mu module. This is a part of the project that is important while I seem to constantly overlook.

A buck converter is required to step the 12-21v down to 12v for the fan. A standard four-pin connector like ones found on computer motherboards is used to connect the fan.

