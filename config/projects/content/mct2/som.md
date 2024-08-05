MediaCow Touch 2 surrounds the use of the recently-released LattePanda Mu system on module that makes use of the Intel Processor N-series. 

## Specifications
As part of the [LattePanda Mu Free Trial Event](https://www.lattepanda.com/blog-323168.html) that I entered in May 2024, I received a LattePanda Mu and a Lite Carrier Board on July 1, 2024 which I am eternally grateful for. 

This specific unit going into the first MediaCow Touch 2 prototype uses the Intel Processor N100 x86-64 SoC with 8GB of LPDDR5 memory. The module itself that is planned to go into the first prototype was given the [codename](../../codenames/) "Cyclobutane" for multiple reasons.

Quick Specs of "Cyclobutane":
- CPU: [Intel Processor N100](https://ark.intel.com/content/www/us/en/ark/products/231803/intel-processor-n100-6m-cache-up-to-3-40-ghz.html) with four Alder Lake E-cores at 800MHz to 3.4GHz and 6MB of cache
- GPU: Intel UHD Graphics
- Memory: 8GB LPDDR5-4800 provided by a single [Samsung K3KL3L3BCM-BGCT](https://semiconductor.samsung.com/dram/lpddr/lpddr5x/k3kl3l30cm-bgct/)
- Storage: 64GB Samsung eMMC

Though the N100 is intended as a low-power mobile solution for devices such as Chromebooks, it is significantly more powerful than any ARM-based SoC I could find on a System on Module, including the Rockchip RK3399 on the Graperain GR3399 that was originally going to be used in the project and even more than the Allwinner A64 that was present in MediaCow Touch 1.

After testing the System on Module with a [couple games in early July 2024 on provided Windows 11](/blog/mct2_p1/), I determined that the N100's performance would allow MediaCow Touch 2 to be used in place of a laptop and could find a use in day-to-day life. 