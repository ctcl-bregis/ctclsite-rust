MediaCow Touch "Nashville" was a project for building a tablet computer.

# Planning
MediaCow Touch's development started presumably in early October of 2020 on a Lenovo ThinkServer TS200v (later codenamed "Polyoxybenzylmethylenglycolanhydride") that I was currently using as a dedicated workstation device. 

No 3D design or EDA was done for the project and the only planning I had was in LaTeX-formatted documents. 

The Banana Pi team's support during development was crucial for the success of the project. 

# Hardware
The device made use of off-the-shelf components and parts from other devices. Assembly of the device was rushed so some repairability aspects were overlooked.

## Case
The case was built from plywood reused from a 50-60's era phono radio system. The front bezel was reused from a digital photo frame and consisted of (presumably) solid aluminum. The case was put together using hot glue and wood screws.

The end result was quite heavy and possibly dangerous, usually requiring two hands to hold the device. 

## System
The device utilized the Banana Pi M64 Single Board Computer. This board was chosen over others at the time because the board had a connector for the AXP803's battery interface. Battery management has been the main challenge for these projects because I did not understand that much about battery management at the time. 

Another reason why I used the M64 is that the Allwinner A64 was used in the [PinePhone](https://pine64.org/devices/pinephone/), a device with similar a goal, released in early 2020. 

The specific unit had:

- Allwinner Technology A64
- 2GB of DDR3 memory with 4x SK hynix H5TQ4G83CFR-RDC
- 8GB eMMC storage with a Samsung KLM8G1GETF-8041
- X-Powers AXP803 PMIC

## Keypad Controller
A button pad was added to the project as more of an afterthought. I used a Teensy LC microcontroller board and a keypad consisting of five buttons soldered to a perf board to make an HID keyboard device that sends hotkeys to Android for volume, back, apps view and home. 

I did not know how to modify or even understand the device tree or any internals of Android to have the buttons connected to the GPIO of the M64 for implementing these buttons properly. 

## Display
The device uses a 7" LCD with capacitive touch provided by Banana Pi for use with the M64 and other compatible boards. Rubber padding was used to mount the display panel between the outer metallic frame and wooden case. Especially upon waking, visible artifacts are all over the display, this could likely be related to a bad connection.

# Software
Sometime in November 2020, I ended up having to use Allwinner's PheonixSuite software to flash the Android 7.0 image to the device's eMMC storage. As I did not know anything about building Android or embedded images at the time, an image provided by Banana Pi was used.

# Legacy 
On February 3, 2024, the device was booted for the first time since presumably September 2022 and all functionality continued to be present.

The device may find use in the development of [MediaCow Touch 2 "Paris"](../mct_2/)