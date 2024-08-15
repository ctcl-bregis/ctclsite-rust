### Audio
As the MediaCow series altogether was originally audio-oriented, audio quality still remains as a priority in device design.

#### Headphone Jack
Even though in recent times, I rarely use the headphone jack on any devices as my headphones (Sennheiser Momentum 3) have an integrated USB codec that I connect to a PC or laptop through a USB cable and has Bluetooth that I use with my phone (despite the phone having a headphone jack which is why I even picked it out in the first place), the device will have a headphone jack as I believe most computers, including smartphones, should have one unless it absolutely cannot be included in the design.

MediaCow Touch 2 will include a standard 3.5mm audio jack that likely supports an extra connection for a microphone (TRRS connector) which many phones and laptops have support for. 

#### CODEC
The Audio CODEC would make use of the HD Audio interface exposed by the LattePanda Mu. This makes hardware design more simple in multiple ways. Notably there is no need for an I2C connection.

##### CODEC Selection
A Tempo Semiconductor 92HD95B is used as the Audio CODEC IC. The reason why I do not use a Realtek CODEC like the LattePanda Mu Full Carrier board is because many Realtek components are nearly impossible to source and receive support for as a hobbyist, this includes audio CODEC ICs. Tempo appears to be one of the few options for HD Audio CODECs that I could easily find on retailers such as Mouser Electronics.

It is likely that the 92HD95B uses IP from SigmaTel as IDT acquired SigmaTel's PC Audio division in 2006 then later in 2013, Tempo Semiconductor spun off from the resulting IDT PC Audio division. My experience with sound quality from SigmaTel CODECs was great, even with older components such as on a Creative SoundBlaster 32 that I used in ["Polyethylene Terephthalate"](/projects/pc_pet/) and now ["Polypropylene"](/projects/pc_pp/).