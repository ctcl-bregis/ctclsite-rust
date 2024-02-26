MediaCow Touch "Nashville" was a project for building a tablet computer. Planning for the project started in October 2020 and the device was built from November 22-24, 2020. At the time, it was my most advanced project, taking thourough planning.

# Planning
In 2020, this was a massive undertaking given my skillset at the time. All of the planning was done within LaTeX formatted documents. 

The Banana Pi team was greatly helpful during the planning and assembly stage of the project.

# Hardware

## Case
The case was built from plywood reused from a 50-60's era phono radio system. The front bezel was reused from a digital photo frame and consisted of (presumably) solid aluminum. The case was put together using hot glue and wood screws.

The end result was quite heavy and possibly dangerous, usually requiring two hands to hold the device. 

## System
The device utilized the Banana Pi M64 Single Board Computer. This board was chosen over others at the time because the board had a connector for the AXP803's battery interface. Battery management has been the main challenge for these projects because I did not understand that much about battery management at the time. 

The specific unit had:

- Allwinner Technology A64
- 2GB of DDR3 memory with 4x SK hynix H5TQ4G83CFR-RDC
- 8GB eMMC storage with a Samsung KLM8G1GETF-8041
- X-Powers AXP803 PMIC

## Keypad Controller
A button pad was added to the project as more of an afterthought. I used a Teensy LC microcontroller board and a keypad consisting of five buttons soldered to a perf board to make an HID keyboard device that sends hotkeys to Android for volume, back, apps view and home. 

I did not know how to modify or even understand the device tree or any internals of Android to have the buttons connected to the GPIO of the M64 for implementing these buttons properly. 

## Display
The device uses a 7" LCD with capacitive touch provided by Banana Pi for use with the M64 and other compatible boards. Rubber padding was used to mount the display panel between the outer metallic frame and wooden case.

# Software
Sometime in November 2020, I ended up having to use Allwinner's PheonixSuite software to flash the Android 7.0 image provided by Banana Pi to the device's eMMC storage.

# Legacy 
On February 3, 2024, the device was booted for the first time since presumably September 2022 and all functionality continued to be present.
