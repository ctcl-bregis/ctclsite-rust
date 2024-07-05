
# Projects

## Software

### The Website
This week, a few things changed with the website.

#### Hosting
On May 21, 2024, I have switched hosting of the website from Vultr to Oracle Cloud for free hosting. The "Always Free" options were either a quad-core Ampere ARM with 24GB of RAM or a single core AMD EPYC x86-64 with 1GB of RAM. To avoid having to cross-compile the website executable to ARM64, I went with the AMD EPYC option as the website is compiled on an x86-64 system. The website uses little resources anyway so it did not matter what plan (referred to as "shapes" by Oracle) I chose.

Setup of the VPS is quite different from other shared hosting providers such as DigitalOcean and Vultr. Prior experience with server management was needed to work with VPS setup on Orace Cloud.

|           | Prior Vultr plan                    | Oracle Cloud plan            |
| --------- | ----------------------------------- | ---------------------------- | 
| Cost      | US$6/month                          | Free?                        |
| CPU       | 1 vCPU Unspecified AMD EPYC Zen 2/3 | 1 vCPU AMD EPYC 7551 (Zen 1) |
| Memory    | 1GB                                 | 1GB                          |
| Storage   | 20GB                                | 45GB (according to `df`)     |
| Network   | 1Gbit/s?                            | 480Mbit/s                    |
| Bandwidth | 500GB/month                         | ???                          |

For no cost, a 1GB RAM AMD EPYC instance is great.

The website now uses Ubuntu as there was no readily available Debian option.

#### Code
On May 25, 2024, I have started to rewrite the styling and HTML markup of the website as it was starting to become unmaintainable; a total mess.

#### Other
I have started to write a Terms of Service while updating the Privacy Policy. The Privacy Policy covers not just the website but any interaction with myself or any services that I manage.

### Pixel Fonts
This week, from May 22 to May 25, 2024, I have developed several "Pixel" fonts for use on this website though they can find use in other projects.

What made these fonts possible is [a script that I have written in Python](https://github.com/ctcl-bregis/script-dump/blob/main/converters/piskelc2svg.py) that converts the C array source code output of Piskel to SVG files made up of squares with the option of applying the "Union" function to combine the squares.

The pixel font series has [its own project page now](../../fonts/).

## Hardware

### Hardware Development
I continue to work on PCB design projects.

#### Flipper Zero Audio Module
The KiCad version of the Flipper Zero Audio Module 

#### MediaCow Touch 2 "Paris"
On May 23, 2024, I have officially started development of [MediaCow Touch 2 "Paris"](../../projects/mct2/), which I have made multiple attempts to develop and design since 2021. 

What started development this time is the giveaway of LattePanda Mu hardware by LattePanda. The LattePanda Mu has been chosen recently to be used as the compute module in MediaCow Touch 2. 

### PC Hardware
I took to some PC hardware this week.

#### Toshiba Satellite Netbook
On May 24, 2024, I have taken a look at a small Toshiba Satellite laptop that I had since 2017. The laptop has significant importance for multiple reasons. In 2021, the LCD failed suddenly and the device has not been used since. Upon inspection of the system using an external monitor and a live USB, the installation of Lubuntu dates to early April 2020 which means the system has one of my earliest Linux installations. 

#### WBPC "Polyethylene" (Supermicro X8DAi)
On Sunday, May 26, 2024, I have pulled out "Polyethylene" to back up some files from a hard drive. It was the first time that I have powered on the system since August 2023. 

Since Steam and MultiMC was already installed from former tests, I have decided to take a look at gaming performance once again. Just two games were tested: Minecraft 1.20.6 without mods (i.e. Fabulously Optimized) and ULTRAKILL. I have only noticed FPS drops in the latter when there are many particle effects on screen. The game was ran using Proton Experimental as the system has Linux Mint installed.

# Personal
Yet another week that was difficult personally though I have made great developments during the week.

During this week, I have got my license from the DMV and I can now legally drive. I plan to go on an extended road trip for a few days to further give myself practice.

