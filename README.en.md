# RPanel

[Português](README.md) | [English](README.en.md)

**System resource monitoring and process management for Linux.**

RPanel is a desktop application developed to monitor system resources and track running processes, providing information about hardware, the operating system, and resource usage through a graphical interface.

## Features

### Resource Monitoring

- CPU monitoring.
- RAM monitoring.
- Storage monitoring.
- Network monitoring.
- GPU and VRAM monitoring, when available.
- Internet connection indicator.

### System Information

- Operating system information.
- Motherboard model and manufacturer.
- Processor and memory information.
- X11 or Wayland graphical environment identification.
- Firmware information.
- Disk listing and information.
- System uptime.
- Date and time.

### Process Management

- List of running processes and applications.
- Application identification and icons.
- CPU and memory usage monitoring.
- Disk and network usage information.
- Ability to terminate processes.

### Steam (Still Experimental)

- RPanel includes mechanisms for identifying Steam games.

## Compatibility

RPanel was developed for Linux and tested on different distributions and desktop environments. Most tests were performed in virtual machines.

| Distribution           | VM  | Result                              |
| ---------------------- | --- | ----------------------------------- |
| Ubuntu 26.04           | No  | Tested                              |
| Kubuntu 26             | Yes | Tested                              |
| Lubuntu 26             | Yes | Tested                              |
| Xubuntu 26             | Yes | Tested                              |
| Ubuntu MATE            | Yes | Partial compatibility               |
| Linux Mint 22 MATE     | Yes | Pending                             |
| Linux Mint 22 Cinnamon | Yes | Compatible, with interface bugs     |
| Zorin OS               | Yes | Tested                              |
| elementary OS          | Yes | Tested                              |
| Pop!_OS                | Yes | Tested                              |
| Debian 13              | Yes | Tested                              |
| Kali Linux             | Yes | Tested                              |

Compatibility may vary depending on the desktop environment, session services, and system configuration.

## Technologies Used

- **Rust:** processing and operating system integration.
- **React:** user interface development.
- **Tauri 2:** desktop application framework.

## Installation

Visit the [Releases](https://github.com/BrunoBarelaFernandes/RPanel/releases) section to find available RPanel versions.

Download the package compatible with your Linux distribution and install it.

## Future Vision

RPanel is still in its early stages of development. The features and resources currently available represent only the beginning of the project.

The implementation of all planned features and ideas is expected to be completed by version **3.0.0**, when RPanel should offer a more complete and robust experience, closer to the project's original vision.

Future plans include new monitoring features, real-time performance graphs, improvements to process identification, and an increasingly complete user experience.

## Developer

**Bruno Barela Fernandes**

- GitHub: [BrunoBarelaFernandes](https://github.com/BrunoBarelaFernandes)
- Project: [RPanel](https://github.com/BrunoBarelaFernandes/RPanel)

## Current Version

**1.0.0**

---

[Português](README.md) | [English](README.en.md)
