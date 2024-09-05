
## Introduction
Battery and power management has historically been the main challenge for these projects.

However, with MediaCow Touch 2, battery management itself has not been that difficult and instead the implementation of USB Power Delivery has been one of the most challenging parts of the project.

## Battery Management
One of the goals for MediaCow Touch 2's design is to feel safe about traveling with the device. MediaCow Touch "Nashvlle" was actually dangerous to travel with due to how the battery was mounted.

### Battery Pack
After the extensive search for an off-the-shelf battery pack that would be suitable for this project, I eventually decided to use a replacement battery meant for a specific laptop. Making use of a laptop battery usually means that a critical part, the fuel gauge IC, is integrated in the battery pack along with protection and cell balancing circuitry. As result, the only IC needed to make use of the battery pack is the charger.

Currently, the plan is to use an HP FM08-compatible battery pack.  The FM08 uses either a 12-pin or 16-pin connector, MediaCow Touch 2 is designed for use with the 16-pin version.

The layout is 4S2P; 4-serial, 2-parallel that yields a voltage range of 12v to 16.8v. 

### Battery Charger
The battery charger IC selected is the LTC/Analog Devices LTC4162-LAD.

#### Battery Charge Limiter Feature
A feature found on some smartphones and laptops that I first experienced with the ASUS Zenfone 9 is the ability to limit the battery charge to a certain percentage to prolong battery cell life. 

I noticed on the ASUS Zenfone 9 that if the phone was powered off or the user has not logged in after booting, the system allows the battery to charge to 100%. This is likely due to the charge limiter feature being managed by a process on Android instead of an embedded controller or the battery charger IC. On MediaCow Touch 2, the plan is to have the battery charge limiter implemented with SMEC instead of an operating system process. 

Implementation of this feature through SMEC may not be difficult as no changes to the hardware are needed. 

This feature may be implemented using [SMEC](../ec/) independent of the operating system. 

## USB Power Delivery
The plan is to be able to have the device be able to charge from one of the two USB Type-C ports. The TPS65988 makes this possible. 

## Power Rails
This section describes power distribution on the carrier board.

Section hierarchy follows the hierarchy of connections.

### VSYS
VSYS is the main power supply rail.

When on battery, the supply is expected to be within 12 to 16.8 volts. When powered from USB Type-C, VSYS is expected to be 20 volts.

#### VSYS_SOM
VSYS_SOM is VSYS but switched using a load switch IC, specifically an Infineon BTS7006-1EPP. It is expected to be within 12 to 20 volts. As stated in the name, this power supply is specifically used by the LattePanda Mu (System on Module).

This supply is unique in that it uses an analog signal for reporting data to the embedded controller.

##### SOM_IO_3V3
SOM_IO_3V3 serves as a reference voltage for various interfaces including SMBus/SMLink, QSPI. It is also the suppply voltage for the external BIOS SPI flash memory IC.

The external SPI flash IC for the BIOS is the only SPI flash IC that does not have a Schottky diode on its power input as it is socketed and is intended to be flashed off board.

#### M2_3V3
M2_3V3 is a 3.3v power supply shared between the M.2 Key E and M.2 Key M slots.

#### USB_5V
A 5V supply is shared between the four USB Type-A ports. 

#### FAN_12V
FAN_12V is the supply voltage for the internal cooling fan. 

#### TCP0_PPHV
TCP0_PPHV is a source voltage for USB Power Delivery. It is a programmable with a voltage output between 5 and 20 volts. 

#### TCP1_PPHV
TCP0_PPHV is a source voltage for USB Power Delivery. It is a programmable with a voltage output between 5 and 20 volts. 

#### OLED_VDD
OLED_VDD is a 3.3v power supply for logic circuits on the OLED display.

#### OLED_VOLED
OLED_VOLED is a 12v power supply for the OLED display. 

#### KP_VDD
This is the supply voltage for the keypad board that feeds directly to the FPC connector.

#### SMEC_PREG
This is the output of the SY21019 buck converter that is to be stepped down further by LDOs. The SY21019, in this case, is used as a pre-regulator.

##### SMEC_VDD
This is the main supply voltage for the embedded controller, SMEC. The power supply is also used by the two external oscillators, an SiT1602B and SiT1533.

As seen in the datasheet for the STM32L496xx series (DS11585 Rev 19), page 125, the maximum current expected to be used by the MCU is 150mA. 

The power usage of the two oscillators are tiny; the SiT1602B is expected to use less than 5mA at 3.3v and the SiT1533 is expected to use less than 5uA at 3.3v. 

##### SMEC_VDDA
A dedicated LDO is used for the ADC reference voltage on the microcontroller.

##### ICM_1V8
Another LDO is required for the 1.8v input of the ICM-20948.
