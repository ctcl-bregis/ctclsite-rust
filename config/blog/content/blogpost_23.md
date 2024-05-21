

# Projects
This was another slow week for personal projects.

## Software

### Website
I have started to make significant aesthetic changes to the website.

#### Pixel-12x12
Pixel-12x12 is a font that I put together throughout the week. It was made possible by a Python script that I wrote that converts the C array output of Piskel to SVG files made up of squares. The C output was the closest format to actual data as the other export options would be compressed images or GIF.

The font was drawn in [Piskel](https://www.piskelapp.com), converted with [piskelc2svg.py](https://github.com/ctcl-bregis/script-dump/blob/main/converters/piskelc2svg.py) and was put together with FontForge. There is a plan to have multiple versions, the first one is called "condensed" and each character is up to eight "pixels" wide; 75% width.

### ToDoKiosk
Before I started to switch systems for use with ToDoKiosk described below. I have put out two versions of ToDoKiosk (Rust): 0.2.1 and 0.3.0. 0.2.1 and part of 0.3.0 were mostly styling fixes to be able to fit more items on the screen. 0.3.0 added the ability to override the currently shown task list and automatic refresh interval with URL query strings. An example of a URL that overrides the calendar and autorefresh interval would be `http://127.0.0.1:8000/?cal_name=To-Do&autoreload=30`

## PC Hardware

### SIGN1 "R12"
The desktop system "Dichlorodifluoromethane" was decomissioned on Wednesday, May 15, 2024 and a Raspberry Pi 4B took place of it. This was at first due to the age of the CPU as I planned to run a newer Linux distribution though that ended up not being a concern. I ended up getting a HDMI-VGA converter cable to be able to use common single-board-computers with the VGA monitor that is currently used.

I tried compiling ToDoKiosk for ARM64 though difficulties were met during the compilation stage. I ultimately decided to run ToDoKiosk on the same VM on SVCS1 that has the CalDAV server as ToDoKiosk is web software.

Using a Raspberry Pi 4B has the benefits of lower power usage, lower (audible) noise and simplicity.

## Automotive
On May 15, 2024, I have started to look into solving various electrical issues in a 2005 Chevrolet Tahoe Z71. I pulled out the BCM (Body Control Module) computer from the under the driver side dash to inspect the circuit board. 

The PCB of the module was clean and was in great condition. The chips on board had date codes within Week 1 to 5, 2005 and the module was branded as Delphi. It is believed that this was the original OEM module and it has not been replaced. As just two door switches would intermittently report as ajar when the door was closed, it is most likely that the door switches are the source of the problem.

# Socials
I have not mentioned this in earlier blog posts mostly since I forgot: Starting May 7, 2024, I am taking an extended break from Instagram. From time to time, once a week to once a month, I would check if I have any new messages or important notifications.

# Personal
On Thursday, May 16, 2024, I drove all the way to [2nd Life Inc](https://www.2ndlifeinc.com/) and back for the first time as part of my driver's education. This was not the longest drive I had but it was seen as a great achievement as I have anticipated doing this for the last few years.

On Friday, May 17, 2024 as part of the conclusion of my driver's education, I drove all the way to the QTS Richmond datacenter (formerly Infineon/Qimonda Richmond) and back. At this point, I have earned my driver's license and would be able to drive independently once DMV paperwork is completed.

