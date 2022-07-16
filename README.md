####### ####### ####### ####### ####### ####### ####### ####### #######

 d888b   .d8b.  db      d88888b d88888b  .d88b.  d8888b.  .o88b. d88888b 
88' Y8b d8' `8b 88      88'     88'     .8P  Y8. 88  `8D d8P  Y8 88'     
88      88ooo88 88      88ooooo 88ooo   88    88 88oobY' 8P      88ooooo 
88  ooo 88~~~88 88      88~~~~~ 88~~~   88    88 88`8b   8b      88~~~~~ 
88. ~8~ 88   88 88booo. 88.     88      `8b  d8' 88 `88. Y8b  d8 88.     
 Y888P  YP   YP Y88888P Y88888P YP       `Y88P'  88   YD  `Y88P' Y88888P 
                                                                         
####### ####### ####### ####### ####### ####### ####### ####### #######

1. setup and dependencies

	Galeforce requires Arcropolis (which you likely already have) : https://github.com/Raytwo/ARCropolis/releases
	Please make sure you are using Arcropolis 3.1.1 or newer!


2. Installation Instructions

	Unpack the archive in the root of your sd card
	overwrite files if prompted

3. troubleshooting

	3a. Wavedashes / other features aren't working or are the same as vanilla
	Please make sure arcropolis is installed and the prc files are in the correct location (refer to the arcropolis readme if needed)

	3b. Moveset edits aren't loading / can't turn in midair
	The plugin is probably not installed or loading properly

	3c. Crash at game startup
	the dependencies were removed from the download, to avoid shipping outdated versions.
	Open the following folder on your sd card: atmosphere/contents/01006A800016E000/romfs/skyline/plugins
	you should find the following files:
		-libgaleforce.nro
		-libnro_hook.nro
		-libsmashline_hook.nro

	if a file is missing, you need to download it and move it inside skyline's plugin folder.

	nro hook: https://github.com/ultimate-research/nro-hook-plugin/releases/download/master/libnro_hook.nro
	smashline hook: https://github.com/blu-dev/smashline_hook/releases/download/1.1.1/libsmashline_hook.nro

4. Ressources

	4a. Discord server: https://discord.gg/Ud9WUu4

	4b. Google doc with all the most recent changes: https://docs.google.com/document/d/1OuRHE3QEvbmG4sQTg7z-njEbjdC-lSiPnd8qw4fto1U/edit?usp=sharing
