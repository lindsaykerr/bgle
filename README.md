# BGLE (Batocera Game List Editor)

BGLE offers a user-friendly and efficient method for editing Batocera's game listing metadata. In Batocera, game metadata is stored in a gamelist.xml file located in the emulator directory. BGLE attempts to provide an alternative way to modify this file and adds seamless switching between emulator and game listings.

### Background
[Batocera](https://github.com/batocera-linux/batocera.linux) is a Linux distribution for managing game emulators. To play games using an emulator, users must have access to a ROM file of a game they own, which they can place within the emulator's folder. When Batocera starts up, it will list the games, but only by their filenames. When many games are listed under an emulator in this way, it can be difficult to pick out a game from the list without additional identifiers, such as a thumbnail. Fortunately, Batocera supports the display of metadata, that can include the game's name, thumbnail, brief description, and publisher. In Batocera this metadata can be obtained by scraping the internet or editing it manually through the standard Batocera interface. However, editing metadata manually can be a hassle, and this is where BGLE can help.

## DISCLAIMER
At the moment, this software is in the Pre-Alpha stage. Although most of the essential features have been developed, it still has some bugs and lacks some features. These bugs could cause the unintentional overwriting of gamelist.xml files and the loss or modification of data. Therefore, it is recommended to create a duplicate of Batocera's "roms" directory for testing purposes. 

