# Flashback

## 26/01
* Implemented a bunch of fixes: scanline timing, interrupt handling for VBlank mode
* Logo scrolling is working, aka SCX and SCY are being handled properly along with LY
* Logo still garbled
* White screen persists, emulator gets stuck trying to use write_reg and read_reg functions for HL which have not been handled properly

**Next Steps**: Handle the write_reg and read_reg functions for HL properly and trace the program output. Hopefully, it proceeds past the logo scrolling section
---
