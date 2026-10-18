# Installing WoW 1.12.1 data

The iOS app does not include Blizzard's game files. After installing the IPA, copy a legally
obtained WoW 1.12.1 `Data` folder into the app's Documents folder:

1. Open the iPhone **Files** app and choose **Browse**.
2. Open **On My iPhone** and then **WarcraftSurvivors**. If it is not visible yet, launch the
   app once, close it, and check Files again.
3. Copy the complete `Data` folder from your WoW 1.12.1 installation into that location. The
   final path should be `On My iPhone/WarcraftSurvivors/Data` (with files such as `common.MPQ`
   directly inside `Data`). Do not put the `Data` folder inside another `Data` folder.
4. Reopen WarcraftSurvivors. The client searches the app's Documents `Data` folder automatically.

The initial copy can take a while because the game data is large. The iOS app does not bundle or
download the game data. Builds without the data folder can start, but the client cannot load the
WoW world or its game audio.
