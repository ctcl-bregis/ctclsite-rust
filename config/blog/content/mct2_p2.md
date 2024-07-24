Development of MediaCow Touch 2 has been... painful, nonetheless. My limited knowledge has been the largest complication. As this project has been the main thing on my mind since June 24th, I have been starting to deal with burnout and wanting to work on something else though I must not give up on this project and do as much as I possibly can before August 26, 2024. 

## Case
I have made some additions to the case design. One notable change is the addition of standoff mounts for the carrier board along with bolt holes for mounting the LCD holder and cover to the case. There are still some changes I can do in order to decrease the amount of material used and decrease the weight of the device. Such changes are not too important at the moment and would likely come when I am ready to 3D print the case parts.

## Schematics and Drawings
I later decided that drawing block diagrams for a lot of the design I am doing is pointless since most of the data I convey in the diagrams are what I could be doing in schematics. Because of this, I decided to go ahead and start drawing the schematics in KiCAD. 

## Challenges
Most of the challenges faced are related to circuit design

### USB Type-C
USB Type-C has been hard to implement, especially with my zero experience in designing with the technology. Others working with the Mu have been having similar experiences.

I found out about the two 'TCP' interfaces available from the Intel N100 where it has the USB SuperSpeed and SBU mux built in. With the TCP interface, all I need to do is deal with is the USB Power Delivery controller and controlling it with an embedded controller (PMEC). 

I have been looking into many ICs for the USB PD controller itself which communicates with the N100 through some sort of interface called SMLink. I could not find any amount of meaningful information about the protocol but it seems to be used by existing Intel-based laptops such as the [Framework Laptop 13](https://github.com/FrameworkComputer/Framework-Laptop-13/blob/main/Mainboard/Mainboard_Interfaces_Schematic_12th_Gen.pdf). 

My current plan is to use a dual-port USB PD controller that would communicate directly with PMEC (Power Management Embedded Controller) then PMEC would communicate with the USB TCP controller on the Intel N100. This would let me use a larger variety of PD controllers. I have considered multiple chips from TI, Maxim/Analog Devices and Cypress/Infineon. As of writing, I have not picked out an IC that I know for sure I would use in the project. There might be a chance that the IC I would use is the TI TPS65994AD but that is subject to change, quickly.

Myself and others working with the LattePanda Mu are currently trying to figure out how to have common microcontrollers such as the STM32 series communicate with the N100 TCP controller. The LattePanda Mu Full Carrier appears to use the IT8851 controller from ITE. Others that I have worked with considered using that chip as it is known to work with the LattePanda Mu specifically. On Sunday, July 21, 2024, AlphaArea, who is part of the LattePanda Team has stated on Discord that they requested the firmware engineer for documentation about the communication between the IT8851 PD controller and the SMLink bus provided by the module. This is crucial for the success of implementing USB Type-C ports to MediaCow Touch 2 along with others' projects.

### Battery
I continue to have the plan to use an HP FM08 battery pack, however finding a good source for one has been difficult. I want to find a battery pack that I would know would be safe. 

I would have to figure out the pinout of the battery pack myself. Finding a compatible connector is also difficult since there is also no data on that which is understandable as the battery pack is meant to replace an existing one in a specific HP laptop instead of being implemented in a new design. However, the connector appears to likely be a JST ZH-style connector. Though considerably safer, easier and less expensive than building my own battery pack from eight 18650 cells, I still have some uncertainty about using one of these laptop battery packs.

On July 24, 2024, I rediscovered the immense complexity of implementing a BQ25700A battery charger IC. In early 2022, I attempted to use this chip for this project while looking at the schematic for its EVM (Evaluation Module). It is one of those things that I would look at and think to myself "just what did I get myself into". There are some more simple options like the LTC4162-L that I may use instead.

<figure>
    <img src="/static/blog/mct2_p2/bq25700a_evm.webp">
</figure>

### PCB
A major concern with designing a PCB with high speed signals such as PCIe is that properly debugging the design requires equipment that is over US$23,000. I came up with a few ideas to overcome this: for one, just do it right the first time by peer reviewing and thoroughly checking the design. Another idea is to get help from a local university with an extensive engineering department that has such equipment; [the same one I do the datacenter stuff at](../wk08_2024/).

I have not gotten to the step of designing the PCB as I still have so much I need to do on the schematic.