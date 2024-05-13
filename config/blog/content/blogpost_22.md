# Projects

## Hardware
This week I have been getting more used to using KiCad for circuit design.

### Flipper Zero
It has been a week since I received a Flipper Zero device. I have been exploring the possibilities of the device and its software. 

#### Flipper Zero Audio Module
Within this week, I have finalized the circuit design for the Flipper Zero Audio Module. This is the first time I have completed a circuit design in KiCad. 

On Sunday, May 12, 2024, I have noticed that Adafruit discontinued their [I2S headphone audio board](https://www.adafruit.com/product/3678) and the only replacement is a [board using an MAX983567A](https://www.adafruit.com/product/3006). I plan to design a general-purpose version of the Flipper Zero Audio Module; a TLV320DAC3203 breakout board that is basically the same thing as the module but smaller.

### MicroMemory
Following discussion with others online, I have decided to design MicroMemory modules using PSRAM for microcontrollers that support it. Depending on the type of memory used, it may have a different physical connection type.

## Software
I have mostly been doing research in ways to make development of software projects easier.

### Databases
Recently, I have been looking into ways to have applications store data in a non-tabular format.

During the week, I have tried out [ScyllaDB](https://www.scylladb.com/) by hosting it in a virtual machine on [SVCS1 "Levoamphetamine"](../../projects/svcs/). This was done to explore using ScyllaDB as a database for use with my current projects that use databases such as [ContactList, [CAMS](.././projects/cams/) and [SLAG](../../projects/slag/).

One main concern I have with using databases like ScyllaDB is that it appears to be made for much much larger deployments handling petabytes of data instead of small, single-user applications like ContactList. 

MonogDB is an option but has some shortcomings. For example, in the 5.0 release from 2021, MongoDB now [requires AVX instructions](https://www.mongodb.com/docs/manual/administration/production-notes/#x86_64) to be present with x86-64 versions of the server. This would leave out a lot of hardware that is still in use such as Intel Xeon 55xx (Nehalem), 56xx (Westmere) series CPUs. One of my ideologies with developing software is to support as much hardware as possible.

# Personal
The last few weeks had personal difficulties but I am working on ways to improve my life and productivity.

I was quite late on uploading the blog post for the last week. 