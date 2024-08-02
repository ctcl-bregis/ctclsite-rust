#### Front-Facing Camera
The front-facing camera would be in the form of a laptop webcam pulled from either a Chromebook or a laptop. The front-facing camera along with the microphone would be useful in cases such as video chat, video logs and selfie photography (though the device would likely require two hands to hold).

Internally, the webcam module connects to the system through a USB 2.0 link though the microphone would have an output for a PDM signal that would be used by the audio CODEC IC on the carrier board.

#### Back-Facing Camera
The back-facing camera would be in the form of a camera module with a MIPI CSI interface. I discovered that the pinout on the LattePanda Mu is the same as the 22-pin camera connector found on some Raspberry Pi single-board computers. As result, I would most likely use a camera module intended for such model of Raspberry Pi.

Since the camera would be connected directly to the LattePanda Mu System on Module, it is most likely that the hardware kill switch for cameras and microphones would not effect the back-facing camera. This is done to simplify the design of the first carrier board revision. If the ability to disable the back-facing camera is needed in the future, there can be another board revision that uses the other MIPI CSI interface available to the carrier board.