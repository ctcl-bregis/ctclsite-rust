On April 12, 2024, I have started the development of a module that adds an audio output for the Flipper Zero. Development started after I reviewed the datasheet for the [STMicroelectronics STM32WB55](https://www.st.com/en/microcontrollers-microprocessors/stm32wb55rg.html) series and verified that pins required for SAI (Serial Audio Interface) were available through the GPIO on the Flipper Zero.

The first design made use of the [Texas Instruments TLV320AIC3253](https://www.ti.com/product/TLV320AIC3253) and was designed in less than a couple hours. This design was shelved due to the cost of the TLV320AIC3253 being between US$7-11 per unit.

On April 17, 2024, I came up with another design using the [Analog Devices/Maxim Integrated MAX9867](https://www.analog.com/en/products/max9867.html) that is about US$2-4 per unit and seems to be better suited for the application. 

Yet again, I made another revision now usiong the [Texas Instruments TLV320DAC3203](https://www.ti.com/product/TLV320DAC3203) on April 19, 2024.

Potential applications:

- Displaying images on an oscilloscope; oscilloscope music
- Use of the Flipper Zero as a Portable Media Player (PMP)
- Testing of audio equipment

I do not have a Flipper Zero on hand as of April 19, 2024, testing of the design is planned to be done using a micrcontroller board or evaluation kit with an I2S interface.