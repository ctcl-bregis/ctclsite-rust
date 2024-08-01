## Challenges

### Time
Pressure is growing more and more to finish this project, as of writing; July 31, 2024, I just have 26 days left to get a finished design. Even back in June this year, where I just started this project, I felt as I was faced with an impossible task. Now it feels even more difficult given the short period of time I must get this project done in. 

I must squeeze every little bit of tenacity out of myself and not give up. As mentioned in previous posts, I no longer expect to have a physical prototype by August 26, 2024. 

### OLED Display
Starting July 27, 2024, MediaCow Touch 2 would contain a small 128x64 OLED display on the side of the case. I know it seems like it would greatly overcomplicate the project, which it would to an extent, but it would be quite useful and may be worth implementing. 

The main function of the OLED is to display device status and diagnosis. During development, it could be used to diagnose circuits by displaying output from PMEC/SMEC.

### Button Panel
On July 31, 2024, I found out that I must have a separate PCB for the button panel to be able to fit everything. It is certainly annoying that I have to design a separate PCB for this but it would be two layers at most and small. As result, I would use Omron B3F-1052-G tactile switches with B32-1060 caps for the buttons and C&K OS102011MS2QN1C for the slide switches.

There would be a total of 12 tactile buttons and two slide switches. The button PCB would contain just one IC being the TCA9535 I/O Expander. A single 8-pin FPC would connect the button board to the carrier board.

The schematic and PCB for the button board was completed on July 31, 2024.

### Power Distribution
Power distribution would be a complex part of the device as many different devices on the carrier board could be powered on and off. 

#### Mu Power Switch
For the switching power to the Mu, an [Infineon BTS7006-1EPP](https://www.infineon.com/dgdl/Infineon-BTS7006-1EPP-DataSheet-v01_10-EN.pdf?fileId=5546d462636cc8fb016421dbf056112b) may be used. Though rated for automotive applications, which seems unavoidable for load switches that support such high voltages, it may be useful in this project. The IC can supposedly switch loads up to 28 volts and 12.5 amps. 

An interesting feature of the BTS7006-1EPP that many other Infineon load switches have is the IS pin. In the design, the plan is to use a 1.2KOhm resistor from IS to ground.

### Embedded Controller
Finally, on Monday, July 29, 2024, I decided to combine the functions of the two embedded controllers. The embedded controller would be now just be refered to as "SMEC". 

The microcontroller selected for SMEC is the STM32U5A5ZJT. 

### Firmware
On July 31, 2024, [KunYi Chen](https://kunyichen.wordpress.com) ([GitHub](https://github.com/KunYi)) has seemed to have successfully ported [Coreboot](https://www.coreboot.org) to the LattePanda Mu. This has saved me potentially massive amounts of work that I would have needed to do when I reach the software development stage.

## Documentation
On July 28, 2024, the idea to use Sphinx was dropped due to potential difficulties in integrating it into this website along with confusion on the implementation of custom themes. Instead, I came up with "documentation" feature that can be used on most pages. As I deal with exhaustion and the ever growing pressure behind the completion of MediaCow Touch 2's hardware designs, the documentation page type's implementation is really hacky and just supports two page levels for the time being. 

Currently, the only page that makes use of this new documentation feature is the [MediaCow Touch 2 Project Page](../../projects/mct2/). It may find use in other pages I start working on more projects with the same amount of complexity, namely CAMS and MediaCow Touch Mini SP7021. At that point, I may add the feature to have more document levels which is not difficult as Serde JSON appears to easily support recursive strucutres.

## Other Things
I went out to basically do research by looking at existing devices that make use of the same parts that I plan to use in MediaCow Touch 2.

### Chromebooks and Laptops
On July 30, 2024, I went to an OfficeMax to look at various modern laptops and chromebooks.

To my luck, I did find a Chromebook with an Intel Processor N100; the same SoC used by the LattePanda Mu used in this project. I was not able to get much data about the embedded controller through `chrome-ec` in `chrome://system`.

Another Chromebook, which used an unknown 8-core ARM SoC, had an Innolux AAS IPS LCD though the backlight did not get nearly as bright as it should so it was difficult for me to compare it with other displays.

I took many photos of the `chrome://system` readout of both units though significantly more of the N100-based unit. 

It seems that in these newer versions of ChromeOS, the `memory_spd_info` field no longer exists or was renamed like `monitor_info`. As result, I was unable to get any information on the DRAM in these devices, which does not really matter in this case. 

#### Gallery - Lenovo N100 Chromebook

<figure>
    <img src="/static/blog/mct2_p4/len_n100_codec.webp">
    <figcaption>Audio info 1</figcaption>
</figure>

<figure>
    <img src="/static/blog/mct2_p4/len_n100_codec_2.webp">
    <figcaption>Audio info 2</figcaption>
</figure>

<figure>
    <img src="/static/blog/mct2_p4/len_n100_cpuinfo.webp">
    <figcaption>CPU info</figcaption>
</figure>

<figure>
    <img src="/static/blog/mct2_p4/len_n100_emmc.webp">
    <figcaption>Storage info</figcaption>
</figure>

<figure>
    <img src="/static/blog/mct2_p4/len_n100_ints.webp">
    <figcaption>Interrupts</figcaption>
</figure>

<figure>
    <img src="/static/blog/mct2_p4/len_n100_lsusb.webp">
    <figcaption>lsusb output</figcaption>
</figure>

<figure>
    <img src="/static/blog/mct2_p4/len_n100_monitorinfo.webp">
    <figcaption>Display information</figcaption>
</figure>

<figure>
    <img src="/static/blog/mct2_p4/len_n100_psu.webp">
    <figcaption>Power info 1</figcaption>
</figure>

<figure>
    <img src="/static/blog/mct2_p4/len_n100_psu_2.webp">
    <figcaption>Power info 2</figcaption>
</figure>

<figure>
    <img src="/static/blog/mct2_p4/len_n100_typec.webp">
    <figcaption>USB Type-C 1</figcaption>
</figure>

<figure>
    <img src="/static/blog/mct2_p4/len_n100_typec_2.webp">
    <figcaption>USB Type-C 2</figcaption>
</figure>

#### Gallery - HP ARM Chromebook

<figure>
    <img src="/static/blog/mct2_p4/hp_cb_monitorinfo.webp">
    <figcaption>Display information</figcaption>
</figure>

<figure>
    <img src="/static/blog/mct2_p4/hp_cb_soc.webp">
    <figcaption>ARM SoC info 1</figcaption>
</figure>

<figure>
    <img src="/static/blog/mct2_p4/hp_cb_soc_2.webp">
    <figcaption>ARM SoC info 2</figcaption>
</figure>

<figure>
    <img src="/static/blog/mct2_p4/hp_cb_mct2p1.webp">
    <figcaption>MediaCow Touch 2 - Part 1 webpage shown</figcaption>
</figure>

### Search for chips... in a scrapyard
On July 29 and July 30, 2024, I went to Chesterfield Auto Parts, which has a convenient location that takes me less than 10 minutes to reach.

Instead of the usual reason of me going there to learn how cars work, I was specifically searching for any module that may contain a modern Infineon BTS-series load/high-side switch. Due to Infineon's massive presence in the automotive electronics market, I could expect to find one of these ICs in any vehicle may it be Japanese, American or especially, German. It appears that these switches, especially with the voltages that are used by the LattePanda Mu, are almost always automotive-grade parts. I did not really need to look in a scrapyard to find an example of how load switches are used on electronic modules but it was something interesting for me to do with the project instead of the mind-numbingly boring work I have been doing.

I rememebered at the last moment that the under-dash BCM on the 2005 Chevy Tahoe Z71 that I have started to work on earlier this year does have an Infineon BTS-series switch, specifically the BTS5215L, however it lacks the more advanced features found on the BTS7006-1EPP that I plan to use in MediaCow Touch 2. 

On the first day, I found a 2004 Volvo XC70 with the ECU and TCM still installed. I removed the covers but I was stopped by the need for a specific tool that is required to properly remove the modules. 

<figure style="width: 100%">
    <img src="/static/blog/mct2_p4/volvo_modules.webp">
    <figcaption>2004 Volvo XC70 ECU and TCM</figcaption>
</figure>

On the second day, I found a 2015 Nissan that had much of the front-end removed already though the ECU was still present. The engine was held up by tire rims. During my attempts to remove the ECU, I must have pushed on the engine a little and it shifted, making me jump back. It would have been likely that the engine may have crushed my leg or scrape it in the way where it would cut my life short in an extremely painful way. The ratcheting part of the wrench I was using literally fell apart and it was getting much too dangerous for me to be around the vehicle. By that time, the place was about to close as signified by the honking of car horns. Right before I left, I made sure to close the hood of the Volvo XC70 to prevent (further) water damage to anything inside.

