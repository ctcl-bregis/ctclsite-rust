On April 12, 2024, I have started the development of a module that adds an audio output for the Flipper Zero. Development started after I reviewed the datasheet for the [STMicroelectronics STM32WB55](https://www.st.com/en/microcontrollers-microprocessors/stm32wb55rg.html) series and verified that pins required for SAI (Serial Audio Interface) were available through the GPIO on the Flipper Zero.

The first design made use of the [Texas Instruments TLV320AIC3253](https://www.ti.com/product/TLV320AIC3253) and was designed in less than a couple hours. This design was shelved due to the cost of the TLV320AIC3253 being between US$7-11 per unit.

On April 17, 2024, I came up with another design using the [Analog Devices/Maxim Integrated MAX9867](https://www.analog.com/en/products/max9867.html) that is about US$2-4 per unit and seems to be better suited for the application. 

Yet again, I made another revision now usiong the [Texas Instruments TLV320DAC3203](https://www.ti.com/product/TLV320DAC3203) on April 19, 2024.

Potential applications:

- Displaying images on an oscilloscope; oscilloscope music
- Use of the Flipper Zero as a Portable Media Player (PMP)
- Testing of audio equipment

 As mentioned before, I do not have a Flipper Zero on hand as of April 23, 2024. Testing of the design is planned to be done using a microcontroller board or evaluation kit with an I2S interface.

By request, [the design files for the TLV320DAC3203 module were uploaded to a GitHub repository](https://github.com/ctcl-bregis/flipper-zero-audio-module/) though the design is unreviewed and is not guaranteed to be functional or have an optimal layout as of April 23, 2024.

In May 2024, I have started to redesign the module in KiCad. This new version moves the I2C pins from pins 13 and 14 to 15 and 16.