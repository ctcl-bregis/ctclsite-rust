# Software
Names for software generally follow the guideline that they simply describe what the software does. This minimalistic mindset is also used in the development of the software in which the software is designed to do exactly what it is described to do. This, however, leads to names being quite generic and could be difficult to find in online searches; I obviously was not the first to use the name ['ContactList'](https://github.com/search?q=ContactList&type=repositories).

Often, these names are acronyms, for example:

* [SLAG](../slag/) for "Security, Logging, Analytics, General (purpose)".
* CAMS is currently "CAMS Asset Management System", a recursive acronym. It's prior meaning was "Computer Asset Management System" but was changed to reflect the software's ability to be used in other applications such as automotive and medical asset management.

## Internal Names and Codenames
Software with the same purpose but written in a different programming language are considered a different project.

For example, with SLAG:
* slag-rust "Black-footed Ferret"
* slag-python "Channel Catfish"

## Repository Names
Starting April 1, 2024, all repositories/internal names of projects would have their programming language specified. When a software project is done in another programming language, it is considered another project and is assigned a codename.

On April 1, 2024, these names were changed:
* cams -> cams-rust, existing "cams-rust" repository was deleted as it was empty
* contactlist -> contactlist-python
* slag -> slag-python
* ToDoKiosk -> todokiosk-python

# Hardware
Hardware projects may have some creativity in their naming unlike software projects.

## MediaCow
MediaCow was a name I came up with in late 2018 for a tablet-like portable media player as the spiritial successor to the "Digital Media Locker" project. The original "MediaCow" was basically an Amazon FireTV Stick, battery pack, 7" HDMI LCD in a custom-made wooden box, the device was tested in March 2019 and was a great achievement at the time.

### MediaCow Touch
MediaCow Touch is a device that is basically a tablet running Android. These are the most complex projects in the series while having the most processing power and memory size.

MediaCow Touch was inspired by the name "iPod Touch" by Apple and the name was chosen for the first [MediaCow Touch "Nashville"](../mct/) project in October 2020.

MediaCow Touch devices are in a series, with the original being MediaCow Touch and the next being [MediaCow Touch 2](../mct2/). 

### MediaCow Touch Mini
MediaCow Touch Mini are just like MediaCow Touch but are physically smaller and usually use lower-power SoCs (system on chip).

In the case of multiple MediaCow Touch Mini devices, names of these devices may have the name of the SoC used in the project. For example, a device using the Samsung S5P6818 may be called MediaCow Touch Mini S5P6818.

### MediaCow Micro
MediaCow Micro are much smaller devices that use microcontrollers instead of SoCs or microprocessors and do not use an operating system kernel.

In 2021, I had the idea of creating 26 different devices following the alphabet; MediaCow Micro A to Z. The letter describing the microcontroller used.
