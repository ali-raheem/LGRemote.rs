# LGRemote

LG TVs of the 2012 era (before WebOS) allowed control via the network, this command line program allows you to issue commands to a compatible TV on your network.

If this project doesn't meet your needs check the Resources/References section for several others.

# Usage
TVs can be identified through [SSDP] which is not yet supported. This is required to find the IP addresses of a TV.
1. `LGRemote IPaddress` will cause the TV to display the pairing key.
2. `LGRemote IPaddress Key Command` will issue the command to the TV.

# Compatible Commands
| Command | Key | Notes |
| --------|-----|------ |
| POWER | 1 |  |
| NUM_0 | 2 |  |
| NUM_1 | 3 |  |
| NUM_2 | 4 |  |
| NUM_3 | 5 |  |
| NUM_4 | 6 |  |
| NUM_5 | 7 |  |
| NUM_6 | 8 |  |
| NUM_7 | 9 |  |
| NUM_8 | 10 |  |
| NUM_9 | 11 | |
| UP | 12 |  |
| DOWN | 13 |  |
| LEFT | 14 |  |
| RIGHT | 15 |  |
| OK | 20 |  |
| HOME | 21 |  |
| MENU | 22 |  |
| BACK | 23 |  |
| VOLUME_UP | 24 |  |
| VOLUME_DOWN | 25 |  |
| MUTE | 26 |  |
| CHANNEL_UP | 27 |  |
| CHANNEL_DOWN | 28 |  |
| BLUE | 29 |  |
| GREEN | 30 |  |
| RED | 31 |  |
| YELLOW | 32 |  |
| PLAY | 33 |  |
| PAUSE | 34 |  |
| STOP | 35 |  |
| FF | 36 |  |
| REW | 37 |  |
| SKIP_FF | 38 |  |
| SKIP_REW | 39 |  |
| REC | 40 |  |
| REC_LIST | 41 |  |
| LIVE | 43 |  |
| EPG | 44 |  |
| INFO | 45 |  |
| ASPECT | 46 |  |
| EXT | 47 |  |
| PIP | 48 |  |
| SUBTITLE | 49 |  |
| PROGRAM_LIST | 50 |  |
| TEXT | 51 |  |
| MARK | 52 |  |
| _3D | 400 |  |
| _3D_LR | 401 |  |
| DASH | 402 |  |
| PREV | 403 |  |
| FAV | 404 |  |
| QUICK_MENU | 405 |  |
| TEXT_OPTION | 406 |  |
| AUDIO_DESC | 407 |  |
| NETCAST | 408 |  |
| ENERGY_SAVE | 409 |  |
| AV | 410 |  |
| SIMPLINK | 411 |  |
| EXIT | 412 |  |
| RESERVE | 413 |  |
| PIP_CHANNEL_UP | 414 |  |
| PIP_CHANNEL_DOWN | 415 |  |
| PIP_SWITCH | 416 |  |
| APPS | 417 |  |

# Resources/References
Unfortunately official documentation doesn't seem to be available any longer but several libraries and scripts are available. node-lgtv-api[4]  is by far the most complete and useful project.
1. https://github.com/ubaransel/lgcommander
2.  https://github.com/grieve/python-lgtv
3. http://dorchain.net/~joerg/code/lg.py
4. https://github.com/timmson/node-lgtv-api - most useful
5. https://developer.lgappstv.com/TV_HELP/index.jsp?topic=%2Flge.tvsdk.references.book%2Fhtml%2FUDAP%2FUDAP%2FAnnex - defunct

# License
MIT licensed as that is in keeping with previous projects.
