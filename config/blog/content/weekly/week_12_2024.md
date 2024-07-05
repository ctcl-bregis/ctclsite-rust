
# Projects
As usual, software projects are given the most attention out of other projects. However, this week, I have started to do more work on servers and creative projects.

## Software
I have been learning more about Rust as I use it in projects.

### The Website
On Tuesday, March 19, 2024 I registered the domain ctcl.lgbt and it is now the current domain of the website. The former domain, ctcl-tech.com now redirects to ctcl.lgbt. The main benefit of this domain is that it is shorter than the former domain.

Releases 1.5.0, 1.5.1 and 1.5.2 were deployed this week. These updates were mostly optimization with some cosmetic changes.

The cosmetic changes include a "Play/Pause" button on the about page that pauses or resumes all of the section background videos, this can help with CPU usage and accessibility. Icons can now be added to project sections on the projects page just like blog posts. There was an attempt to fix embeds on services such as Discord with 1.5.0 but it did not work, later attempts did not work either.

### SLAG
I have started to work on SLAG again with the Rust programming language but development is still difficult due to my limited knowledge of Rust.

#### Micron
Due to a recent update to the Micron website, how SLAG interacts with the FBGA decoder utility had to be changed siginificantly.

Notably, the utility seems now to send a GET request to this url: `https://www.micron.com/content/micron/us/en/sales-support/design-tools/fbga-parts-decoder/_jcr_content.products.json/getpartbyfbgacode/-/-/-/en_US/-/-/<fbgacode>` (<fbgacode> being the code, e.g. "d9xpf") and returns JSON describing the part. 

An example of the JSON data:
`{"date":"2024-03-23 18:36:06.572","response-code":"200","details":[{"part-number":"MT40A2G8VA-062E:B","part-key":"mt40a2g8va-062e-b","part-name":"DDR4 16G X8 TFBGA","sub-category":"ddr4-sdram","fbga-code":"D9XPF","pageurl":"/products/memory/dram-components/ddr4-sdram/part-catalog/part-detail/mt40a2g8va-062e-b"}]}`

This actually makes development easier since HTML scraping is no longer needed as the JSON data can be parsed with [serde](https://crates.io/crates/serde) or a similar library.

### ContactList
On Friday, March 22, 2024, I started a rewrite of ContactList in the Rust programming language as my knowledge of Rust is still too limited to work on SLAG. Like the website and ToDoKiosk Rust rewrites that I completed recently, ContactList would use Actix.

## Hardware Design
During this week, I have been exploring potential options for hardware design projects.

## PC Hardware
This week, I have started to finally do some more work with the servers.

### [SVCS2 "Lisdexamfetamine"](../../projects/srv_amp/)
On Monday, March 18, 2024, I have installed 64GB of Qimonda IMHH8GP22A1F2C-10F 8GB RDIMMs in SVCS2 "Lisdexamfetamine" as the 128GB configuration using 16GB SK hynix modules was unneeded.

This changes the plan for what memory is going to be used in ["Polyethylene Terephthalate v2"](../../projects/pc_pet2/). Instead, eight extra Qimonda IMHH4GP12A1F1C-13H DDR3-1333 2Rx4 RDIMMs would be used. Two more of these modules were received on March 24, 2024 in anticipation for the project. A configuration of eight of these modules bring the memory size from 64GB to 32GB but better performance is expected due to multiple factors such as the modules being dual rank instead of quad rank and DDR3-1333 instead of DDR3-1066.

These modules are assembled in Malaysia and possibly could have DRAM diffused in Sandston, Virginia though I would not risk damaging the modules by removing the heatsinks to find out.

### [SVCS3 "Dextroamphetamine"](../../projects/srv_amp/)
At the beginning of the week, I have figured out why one of the HP ProLiant Blades, "Dextroamphetamine" did not boot from the RAID controller. It was simply the matter of setting the boot volume in the HP Option ROM Configuration of Arrays interface. This was prevalent since I was given the server in April 2023.

I installed Debian 12 on the system before trying to host [Jan](https://jan.ai) for use by other systems. This was not successful likely due to the CPUs' lack of the AVX2 extension. Later I tried using the Intel SDE software to emulate AVX2 instructions, this worked however processing speed was very slow (0.09t/s) while the CPUs were at full load. For now, until another method of getting LLMs to run on these processors, the server is to be used for other tasks such as remote compilation.

I anticipated running AI models on these blade servers ever since they were given to me in April 2023.

Soon after, I attempted to run Jan locally on [the desktop "Polybutylene Terephthalate"](../../projects/pc_pbt.md) with Linux Mint, this was successful on both the CPU and GPU. The system's AMD Radeon RX 6700 XT was able to be utilized with the experimental Vulkan support. Processing on the GPU was about six times faster than processing on the CPU.

## Creative Work
I have continued to do small amounts of creative work throughout the week.

### Portal 2
For the time being, I no longer provide level design (mapping) services. Within the last couple months, I ceased work on maps for Forbidden Testing Tracks. This is mainly due to the large amount of other projects as seen above and my still limited knowledge of creating maps for Source-based games.

I have started to work on "The Hall" again as I have gotten more ideas for it.

<figure>
    <img src="/static/blog/15/the_hall_march222024_1.webp">
    <figcaption>View of the outside from one side of the hall.</figcaption>
</figure>

<figure>
    <img src="/static/blog/15/the_hall_march222024_2.webp">
    <figcaption>Part of the map not expected to be seen by the player.</figcaption>
</figure>

<figure>
    <img src="/static/blog/15/the_hall_march222024_3.webp">
    <figcaption>Another view of the "outside" part of the map not accessible by the player.</figcaption>
</figure>

Along with "The Hall", I have created more "cave sounds" that were originally meant to be used in a resource pack for Minecraft but seemed to fit this project much better. 

### nonmonolithic
I have continued to take photos and create other content for nonmonolithic though I do not expect to post any content to Instagram any time in the foreseeable future.