
## Introduction
Battery and power management has historically been the main challenge for these projects.

However, with MediaCow Touch 2, battery management itself has not been that difficult and instead the implementation of USB Power Delivery has been one of the most challenging parts of the project.

## Battery Management
One of the goals for MediaCow Touch 2's design is to feel safe about traveling with the device. MediaCow Touch "Nashvlle" was actually dangerous to travel with due to how the battery was mounted.

### Battery Pack
After the extensive search for an off-the-shelf battery pack that would be suitable for this project, I eventually decided to use a replacement battery meant for a specific laptop. Making use of a laptop battery usually means that a critical part, the fuel gauge IC, is integrated in the battery pack along with protection and cell balancing circuitry. As result, the only IC needed to make use of the battery pack is the charger.

Currently, the battery pack that would be used is one that is compatible with the HP FM08 battery pack. The FM08 uses either a 12-pin or 16-pin connector, MediaCow Touch 2 is designed for use with the 16-pin version.

The layout is 4S2P; 4-serial, 2-parallel that yields a voltage range of 12v to 16.8v. 

### Battery Charger
The battery charger IC selected is the LTC/Analog Devices LTC4162-LAD.

#### Battery Charge Limiter Feature
A feature found on some smartphones and laptops that I first experienced with the ASUS Zenfone 9 is the ability to limit the battery charge to a certain percentage to prolong battery cell life. 

This feature may be implemented using [SMEC](../ec/) independent of the operating system. 

Implementation of this feature through SMEC may not be difficult as no changes to the hardware are needed. 

## USB Power Delivery
The plan is to be able to have the device be able to charge from one of the two USB Type-C ports. The TPS65988 makes this possible. 

## Power Rails
This section describes power distribution on the carrier board.

### VBUS
VBUS is the power input rail that goes directly to the battery charger IC. This is the connection after the PD VBUS switch MOSFETs.

### VSYS
VSYS is the name given to the power output of the battery charger IC. The voltage range of VSYS is 12v to 20v which is also the operating range of the LattePanda Mu. 

A load switch is used between the battery charger IC output and the rest of the system. 

All voltages in the system is stepped down from VSYS. The following sections cover voltage rails that are derived from VSYS.

#### TCP_PPHV1
TCP_PPHV1 is the power source for the first USB Type-C port in PD "source" mode.

TCP_PPHV1 supplies a voltage from 5V to 20V specified by SMEC or the PD controller. An MPS MP8859 buck-boost converter is used for up to 20V at 3A.

#### TCP_PPHV2
TCP_PPHV2 is the power source for the first USB Type-C port in PD "source" mode.

TCP_PPHV2 supplies a voltage from 5V to 20V specified by SMEC or the PD controller. An MPS MP8859 buck-boost converter is used for up to 20V at 3A.

#### USB_5V
USB_5V is shared by every USB load switch on the USB Type-A ports.

The Texas Instruments TPS51386 buck converter is used to bring VSYS voltage down to ~5V to be switched individually for each port.

#### M2_3V3
M2_3V3 is the output of yet another TPS51386 buck converter. This supply is shared by both the M.2 Key E and Key M slots. 

#### VSYS_LV
VSYS_LV is a 5V power supply that is stepped down from the 12-20v VSYS. Yet another Texas Instruments TPS51386 is used for this purpose. 

This power supply exists to lower the implementation cost of adding buck regulators for devices using 3.3v. 

##### SOM_IO_3V3
SOM_IO_3V3 is a reference voltage for 3.3V IO on the LattePanda Mu.

##### LAN_3V3
LAN_3V3 is the 3.3v power supply for the Intel I210 Ethernet NIC.

##### SMEC_3V3
SMEC_3V3 is the main supply voltage for the embedded controller. 

##### HDA_PVDD, HDA_CPVDD, HDA_AVDD
Three separate power supplies are used for the audio CODEC IC. 

### VSB
VSB is always available even when the system is powered off; VSYS is switched off. Its purpose is to supply power to SMEC, the OLED display and the keypad which should be available when the system is powered off.

It is the same voltage range as VSYS as it also directly feeds from the battery charger IC power output. The only difference is that it is not switched.

#### SMEC_VDD
This is the main supply voltage for SMEC. It should have a voltage of 3.3v.

#### OLED_VOLED
OLED supply voltage that is within the recommended range of 10-15v for the CFAL12832A-022W OLED display. The voltage is stepped down from VSB through a Richtek RT6200 buck converter.

#### OLED_VDD
OLED logic supply voltage of 3.3v. It is stepped down from VSB through an LDO linear regulator.

### RTC_VBAT
RTC_VBAT is the power output of the on-board coin-cell battery. The battery is shared by both the System on Module and SMEC (Embedded Controller).
