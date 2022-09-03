// Autosplitter for Sonic Mania
// Provides complete support for all versions known to date:
// - Retail (Steam)
// - Retail (Epic store)
// - Decompilations (RSDKv5 and RSDKv5U, both 32bit and 64bit)
// Coding: Jujstme (very loosely on the previous version by ClarisRobyn)
// contacts: just.tribe@gmail.com
// Version: 1.3.1 (Aug 21st, 2022)

state("SonicMania") {}
state("RSDKv5") {}
state("RSDKv5-dx11") {}
state("RSDKv5-dx9") {}
state("RSDKv5U") {}
state("RSDKv5U-dx9") {}
state("RSDKv5U-dx11") {}
state("RSDKv5_x64") {}
state("RSDKv5-dx9_64") {}
state("RSDKv5-dx11_64") {}
state("RSDKv5U_x64") {}
state("RSDKv5U-dx11_64") {}
state("RSDKv5U-dx9_64") {}

startup
{
    // Constants
    vars.CHARACTER_SONIC = 1;
    vars.CHARACTER_KNUCKLES = 4;
    vars.GAMEMODE_MANIA = 0;
    vars.GAMEMODE_ENCORE = 1;
    vars.ALL_EMERALDS = 0x7F;

    // Acts
    vars.Acts = new Dictionary<int, int>[]
    {
        // Mania mode
        new Dictionary<int, int>{
            {  9, 0 }, { 117, 0 }, { 119, 0 }, // Green Hill Act 1
            { 10, 1 }, { 120, 1 }, // Green Hill Act 2
            { 11, 2 }, // Chemical Plant Act 1
            { 12, 3 }, // Chemical Plant Act 2
            { 13, 4 }, // Studiopolis Act 1
            { 14, 5 }, // Studiopolis Act 2
            { 15, 6 }, // Flying Battery Act 1
            { 16, 7 }, // Flying Battery Act 2
            { 17, 8 }, // Press Garden Act 1
            { 18, 9 }, // Press Garden Act 2
            { 19, 10 }, { 122, 10 }, // Stardust Speedway Act 1
            { 20, 11 }, { 21, 11 }, // Stardust Speedway Act 2
            { 22, 12 }, // Hydrocity Act 1
            { 23, 13 }, // Hydrocity Act 2
            { 24, 14 }, { 25, 14 }, // Mirage Saloon Act 1
            { 26, 15 }, // Mirage Saloon Act 2
            { 27, 16 }, // Oil Ocean Act 1
            { 28, 17 }, // Oil Ocean Act 2
            { 29, 18 }, // Lava Reef Act 1
            { 30, 19 }, { 31, 19 }, // Lava Reef Act 2
            { 32, 20 }, // Metallic Madness Act 1
            { 33, 21 }, // Metallic Madness Act 2
            { 34, 22 }, // Titanic Monarch Act 1
            { 35, 23 }, { 36, 23 }, // Titanic Monarch Act 2
            { 37, 24 }, // Egg Reverie
        },

        // Encore mode
        new Dictionary<int, int>{
            { 118, 25 }, // Angel Island Zone
            { 119, 26 }, { 38, 26 }, // Green Hill Act 1
            { 39, 27 }, { 120, 27 }, // Green Hill Act 2
            { 40, 28 }, // Chemical Plant Act 1
            { 41, 29 }, // Chemical Plant Act 2
            { 42, 30 }, // Studiopolis Act 1
            { 43, 31 }, // Studiopolis Act 2
            { 44, 32 }, // Flying Battery Act 1
            { 45, 33 }, // Flying Battery Act 2
            { 46, 34 }, // Press Garden Act 1
            { 47, 35 }, // Press Garden Act 2
            { 48, 36 }, { 122, 36 }, // Stardust Speedway Act 1
            { 49, 37 }, { 50, 37 }, // Stardust Speedway Act 2
            { 51, 38 }, // Hydrocity Act 1
            { 52, 39 }, // Hydrocity Act 2
            { 53, 40 }, // Mirage Saloon Act 1
            { 54, 41 }, // Mirage Saloon Act 2
            { 55, 42 }, // Oil Ocean Act 1
            { 56, 43 }, // Oil Ocean Act 2
            { 57, 44 }, // Lava Reef Act 1
            { 58, 45 }, { 59, 46 }, // Lava Reef Act 2
            { 60, 47 }, // Metallic Madness Act 1
            { 61, 48 }, // Metallic Madness Act 2
            { 62, 49 }, // Titanic Monarch Act 1
            { 63, 50 }, { 64, 50 }, // Titanic Monarch Act 2
        },
    };

    string[][] actsName = {
        // Mania mode
        new string[]
        {
            "Green Hill Act 1", "Green Hill Act 2",
            "Chemical Plant Act 1", "Chemical Plant Act 2",
            "Studiopolis Act 1", "Studiopolis Act 2",
            "Flying Batter Act 1", "Flying Battery Act 2",
            "Press Garden Act 1", "Press Garden Act 2",
            "Stardust Speedway Act 1", "Stardust Speedway Act 2",
            "Hydrocity Act 1", "Hydrocity Act 2",
            "Mirage Saloon Act 1", "Mirage Saloon Act 2",
            "Oil Ocean Act 1", "Oil Ocean Act 2",
            "Lava Reef Act 1", "Lava Reef Act 2",
            "Metallic Madness Act 1", "Metallic Madness Act 2",
            "Titanic Monarch Act 1", "Titanic Monarch Act 2",
            "Egg Reverie"
        },

        // Encore mode
        new string[]
        {
            "Angel Island",
            "Green Hill Act 1", "Green Hill Act 2",
            "Chemical Plant Act 1", "Chemical Plant Act 2",
            "Studiopolis Act 1", "Studiopolis Act 2",
            "Flying Batter Act 1", "Flying Battery Act 2",
            "Press Garden Act 1", "Press Garden Act 2",
            "Stardust Speedway Act 1", "Stardust Speedway Act 2",
            "Hydrocity Act 1", "Hydrocity Act 2",
            "Mirage Saloon Act 1", "Mirage Saloon Act 2",
            "Oil Ocean Act 1", "Oil Ocean Act 2",
            "Lava Reef Act 1", "Lava Reef Act 2",
            "Metallic Madness Act 1", "Metallic Madness Act 2",
            "Titanic Monarch Act 1", "Titanic Monarch Act 2"
        }
    };
    settings.Add("starts", true, "Autostart options");
    settings.Add("smania", true, "Enable auto start in Mania mode", "starts");
    settings.Add("sencore", true, "Enable auto start in Encore mode", "starts");
    settings.Add("resets", true, "Autoreset options");
    settings.Add("saveSelect", true, "Automatically reset when returning to the save selection screen", "resets");
    settings.Add("devMenu", false, "Automatically reset when opening the dev menu", "resets");
    settings.Add("autosplitting", true, "Autosplitting options");
    settings.Add("mania", true, "Mania Mode", "autosplitting");
    settings.Add("encore", true, "Encore Mode", "autosplitting");
    int J = 0;
    for (int i = 0; i < actsName.Length; i++)
    {
        for (int j = 0; j < actsName[i].Length; j++)
            settings.Add(J++.ToString(), true, actsName[i][j], i == 0 ? "mania" : "encore");
    }
}

init
{
    // Declare a basic scanner variable for the sigscan. Don't define it yet because, in case of the decompilation,
    // we need to refer to the game.dll module instead of the mainModule.
    SignatureScanner scanner;

    // Declare a scannerBase that will always use the MainModule.
    // The decompilation needs it in one specific case.
    SignatureScanner scannerBase = new SignatureScanner(game, modules.First().BaseAddress, modules.First().ModuleMemorySize);

    // Try to find game.dll; if it succeeds, it means we are running on the decomp.
    // If it fails we are running a retail version (Steam or Epic Store).
    var gameModule = modules.FirstOrDefault(m => m.ModuleName.ToLower() == "game.dll");
    if (gameModule == null)
    {
        // In case of the retail version, scanner and scannerBase are the same thing
        scanner = scannerBase;
        version = "Retail";
    }
    else
    {
        // In the decompilation most of the sigscanning will be performed inside the game.dll module
        scanner = new SignatureScanner(game, gameModule.BaseAddress, gameModule.ModuleMemorySize);
        version = "Decompilation (" + (game.Is64Bit() ? "64" : "32") + " bit)";
    }

    // Define the classic variables needed for the sigscanning
    vars.watchers = new MemoryWatcherList();
    IntPtr ptr = IntPtr.Zero;
    IntPtr addr = IntPtr.Zero;
    int offset;
    bool isRSDKv5U;  // This is nice to have in any case. The script actually uses it in one specific spot

    // Scanner helper - thanks Ero
    Action<SignatureScanner, int, string> scan = (s, o, p) =>
    {
        ptr = s.Scan(new SigScanTarget(o, p));
        if (ptr == IntPtr.Zero) throw new NullReferenceException("Sigscanning failed!");
        addr = game.Is64Bit() ? ptr + 0x4 + game.ReadValue<int>(ptr) : game.ReadPointer(ptr);
    };

    // For obvious reasons, we need to use separate signatures according to the architecture (32 vs. 64 bit)
    switch (game.Is64Bit())
    {
        case false: // 32 bit
            isRSDKv5U = scannerBase.Scan(new SigScanTarget("3D ???????? 0F 87 ???????? FF 24 85 ???????? A1 ???????? 89")) != IntPtr.Zero;

            // LevelID. Actually a SceneID, it also identifies whenever we are in the main menu
            scan(scanner, 2, "8B ?? ???????? 85 C9 74 1E 80 ?? ?? 03");
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, game.ReadValue<byte>(ptr + 28))) { Name = "LevelID" });

            // In-game timer. Doesn't even need an explanation, though we are not gonna use it directly
            scan(scanner, 1, "A1 ???????? 80 78 ?? 01 0F 85 ???????? FF");
            offset = game.ReadValue<byte>(ptr + 6);
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, offset)    ) { Name = "Status" });
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, offset + 2)) { Name = "Centisecs" });
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, offset + 3)) { Name = "Secs" });
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, offset + 4)) { Name = "Mins" });

            // Health addresses for the two bosses in Egg Reverie
            scan(scanner, 2, "8B ?? ???????? 8B ?? 40 83 ?? ?? 00");
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, game.ReadValue<byte>(ptr +  6), game.ReadValue<byte>(ptr +  9))) { Name = "ER_Monarch_Health" });
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, game.ReadValue<byte>(ptr + 15), game.ReadValue<byte>(ptr + 18))) { Name = "ER_Eggman_Health" });

            // Flag that immeditely tells when we defeated Eggman in Titanic Monarch Act 2
            scan(scanner, 4, "6A 00 C7 80 ???????? 01000000 A1 ???????? C7 ?? ?? 00000000");
            offset = game.ReadValue<int>(ptr);
            scan(scanner, 1, "A1 ???????? 83 C0 78");
            vars.watchers.Add(new MemoryWatcher<bool>(new DeepPointer(addr, offset)) { Name = "TM2_defeated" });

            // Bitmask for the Chaos Emeralds. One bit for every emerald. 0x7F = 7 emeralds.
            scan(scanner, 3, "7D ?? A1 ???????? 8B ?? ?? 83 ?? ?? 7F");
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, game.ReadValue<byte>(ptr + 6), game.ReadValue<byte>(ptr + 9))) { Name = "ChaosEmeralds" });

            // Characters
            scan(scanner, 2, "8B 15 ???????? 8B 4A 04 C1");
            offset = game.ReadValue<int>(ptr + 6);
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, 0)    ) { Name = "GameMode" });
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, offset)    ) { Name = "Character1" });
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, offset + 1)) { Name = "Character2" });

            // The start trigger actually monitors whenever the game reports a successful selection of the save file.
            // It's a janky solution but it works.
            scan(scannerBase, 7, "69 C0 ???????? 05 ???????? 5D C3 CC");
            vars.watchers.Add(new MemoryWatcher<int>(addr + 0x71808 + (isRSDKv5U ? 0xD14 : 0)) { Name = "StartTrigger" });
            vars.StartOffset = 48;
            vars.watchers.Add(new MemoryWatcher<bool>(addr + 0x720AC + (isRSDKv5U ? 0xD24 : 0)) { Name = "ManiaModeSelection" });
            vars.watchers.Add(new MemoryWatcher<byte>(addr + 0x720C0 + (isRSDKv5U ? 0xD24 : 0)) { Name = "SaveSelection_Mania" });
            for (int i = 0; i < 8; i++) vars.watchers.Add(new MemoryWatcher<byte>(addr + 0x146DC + (isRSDKv5U ? 0x25C + 0x460 * i : 0x458 * i)) { Name = "Save_" + i.ToString() });
            vars.watchers.Add(new MemoryWatcher<byte>(addr + 0x7A768 + (isRSDKv5U ? 0xE1C : 0)) { Name = "SaveSelection_Encore" });
            for (int i = 1; i < 4; i++) vars.watchers.Add(new MemoryWatcher<bool>(addr + 0x6F5F4 + (isRSDKv5U ? 0xCD4 + 0x460 * (i - 1) : 0x458 * (i - 1))) { Name = "Encore_Save_" + i.ToString() });
            break;
        default: // 64 bit
            isRSDKv5U = scannerBase.Scan(new SigScanTarget("81 F9 ???????? 0F 87 ???????? 41 8B 8C")) != IntPtr.Zero;

            // LevelID. Actually a SceneID, it also identifies whenever we are in the main menu
            scan(scanner, 3, "48 8B ?? ???????? 48 85 C9 74 2A 80 7A");
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, game.ReadValue<byte>(ptr + 30))) { Name = "LevelID" });

            // In-game timer. Doesn't even need an explanation, though we are not gonna use it directly
            scan(scanner, 3, "48 8B ?? ???????? 80 78 ?? 01 0F 85 ???????? FF");
            offset = game.ReadValue<byte>(ptr + 6);
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, offset)    ) { Name = "Status" });
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, offset + 2)) { Name = "Centisecs" });
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, offset + 3)) { Name = "Secs" });
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, offset + 4)) { Name = "Mins" });

            // Health addresses for the two bosses in Egg Reverie
            scan(scanner, 5, "75 ?? 48 8B 0D ???????? 48 8B 41 ?? 83");
            var reverie = addr;
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(reverie, game.ReadValue<byte>(ptr + 7), game.ReadValue<byte>(ptr + 10))) { Name = "ER_Monarch_Health" });
            scan(scanner, 0, "75 ?? 48 8B 41 ?? 83 78 ?? 00");
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(reverie, game.ReadValue<byte>(ptr + 5), game.ReadValue<byte>(ptr +  8))) { Name = "ER_Eggman_Health" });

            // Flag that immeditely tells when we defeated Eggman in Titanic Monarch Act 2
            scan(scanner, 6, "89 73 ?? 48 8B ?? ???????? C7 ?? ???????? 01 00 00 00");
            vars.watchers.Add(new MemoryWatcher<bool>(new DeepPointer(addr, game.ReadValue<int>(ptr + 6))) { Name = "TM2_defeated" });

            // Bitmask for the Chaos Emeralds. One bit for every emerald. 0x7F = 7 emeralds
            scan(scanner, 5, "7D ?? 48 8B ?? ???????? 48 ?? ?? ?? 83 ?? ?? 7F 7C");
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, game.ReadValue<byte>(ptr + 7), game.ReadValue<byte>(ptr + 10))) { Name = "ChaosEmeralds" });

            // Characters
            scan(scanner, 3, "48 8B ?? ???????? 8B 4A ?? C1 F9 08");
            offset = game.ReadValue<int>(ptr + 6);
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, 0)    ) { Name = "GameMode" });
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, offset)    ) { Name = "Character1" });
            vars.watchers.Add(new MemoryWatcher<byte>(new DeepPointer(addr, offset + 1)) { Name = "Character2" });

            // The start trigger actually monitors whenever the game reports a successful selection of the save file.
            // It's a janky solution but it works.
            scan(scannerBase, 3, "4C 8D ?? ???????? 8B D0 4C 8D ?? ???????? 0F 1F");
            vars.watchers.Add(new MemoryWatcher<long>(addr + 0xD977A + (isRSDKv5U ? 0x1A10 : 0)) { Name = "StartTrigger" });
            vars.StartOffset = 48;
            vars.watchers.Add(new MemoryWatcher<bool>(addr + 0xDA81E + (isRSDKv5U ? 0x1A30 : 0)) { Name = "ManiaModeSelection" });
            vars.watchers.Add(new MemoryWatcher<byte>(addr + 0xDA836 + (isRSDKv5U ? 0x1A30 : 0)) { Name = "SaveSelection_Mania" });
            for (int i = 0; i < 8; i++) vars.watchers.Add(new MemoryWatcher<byte>(addr + 0x26A9A + (isRSDKv5U ? 0x4A0 + 0x868 * i : 0x858 * i)) { Name = "Save_" + i.ToString() });
            vars.watchers.Add(new MemoryWatcher<byte>(addr + 0xEAADE + (isRSDKv5U ? 0x1C20 : 0)) { Name = "SaveSelection_Encore" });
            for (int i = 1; i < 4; i++) vars.watchers.Add(new MemoryWatcher<bool>(addr + 0xD55AA + (isRSDKv5U ? 0x1990 + 0x868 * (i - 1) : 0x858 * (i - 1))) { Name = "Encore_Save_" + i.ToString() });
            break; 
    }

    // Default variables
    current.LevelID = 0;
}

update
{
    vars.watchers.UpdateAll(game);
}

split
{
    // If you're outside Mania mode or Encore mode, there's no reason to continue
    if (vars.watchers["GameMode"].Current > vars.GAMEMODE_ENCORE)
        return false;

    // Find a way to define the current act base on the LevelID provided by the game
    // In order to avoid early splitting, we want to make sure the game is actually loading a new stage,
    // except in cases where the ID can be updated regardless of the game current status

    // For example, in Egg Reverie, the Level ID must be defined directly, as the timer is randomized, so not useful for our purpose
    if (vars.watchers["LevelID"].Current == 37)
    {
        current.LevelID = vars.Acts[vars.watchers["GameMode"].Current][vars.watchers["LevelID"].Current];
    }
    // Otherwise, we want to safely assume a new stage is loading either by the status of 2, or by the timer zeroed
    else if (
        vars.Acts[vars.watchers["GameMode"].Current].ContainsKey(vars.watchers["LevelID"].Current) && (
            vars.watchers["Status"].Current == 2 ||
            (vars.watchers["Status"].Current != 0 && vars.watchers["Mins"].Current + vars.watchers["Secs"].Current + vars.watchers["Mins"].Current == 0)
        ))
        current.LevelID = vars.Acts[vars.watchers["GameMode"].Current][vars.watchers["LevelID"].Current];

    // Special case no. 1: Titanic Monarch Act 2
    if (old.LevelID == 23 || old.LevelID == 50)
    {
        if (vars.watchers["GameMode"].Current == vars.GAMEMODE_MANIA)
        {
            if (settings[old.LevelID.ToString()])
            {
                // If you have all the emeralds and you're playing as Sonic or in Knuckles & Knuckles mode, then you have to split when you reach Egg Reverie
                if (vars.watchers["ChaosEmeralds"].Current == vars.ALL_EMERALDS && (vars.watchers["Character1"].Current == vars.CHARACTER_SONIC || (vars.watchers["Character1"].Current == vars.CHARACTER_KNUCKLES && vars.watchers["Character2"].Current == vars.CHARACTER_KNUCKLES)))
                    return current.LevelID == old.LevelID + 1;

                // Otherwise, the game ends upon defeat of Titanic Monarch's boss
                return vars.watchers["TM2_defeated"].Current && !vars.watchers["TM2_defeated"].Old;
            }
        } else {
            // In encore mode, the only condition is defeating the Phantom Egg
            return settings[old.LevelID.ToString()] && vars.watchers["TM2_defeated"].Current && !vars.watchers["TM2_defeated"].Old;
        }
    }
    // Special case nr. 1: Egg Reverie needs to split when the bosses have been defeated
    else if (old.LevelID == 24)
    {
        if (settings[old.LevelID.ToString()])
        {
            return vars.watchers["Status"].Current == 1 && vars.watchers["ER_Monarch_Health"].Old + vars.watchers["ER_Eggman_Health"].Old > 0
                && vars.watchers["ER_Monarch_Health"].Current + vars.watchers["ER_Eggman_Health"].Current == 0;
        }
    }
    // If not in one of those special cases, then split when progressing from a level to the next one
    else return settings[old.LevelID.ToString()] && current.LevelID == old.LevelID + 1;
}

start
{
    // If you're not in the main menu, there's no reason to continue
    if (vars.watchers["LevelID"].Current != 2)
        return false;

    // If the main start trigger doesn't get involved, again, there's no reason to continue
    if (vars.watchers["StartTrigger"].Current != vars.watchers["StartTrigger"].Old + vars.StartOffset)
        return false;

    if (vars.watchers["ManiaModeSelection"].Current)
    {
        // First, check if you disabled autostart in Mania Mode
        if (!settings["smania"])
            return false;

        // Then, first check if your save selection is not out of bounds
        if (vars.watchers["SaveSelection_Mania"].Current > 8)
            return false;
        
        // If the no-save option is selected, there's no need to check the save
        if (vars.watchers["SaveSelection_Mania"].Current == 8)
            return true;
        // If you select a save file, make sure you selected an empty zone or Green Hill 1. Works for both new games and NG+
        else
            return vars.watchers["Save_" + vars.watchers["SaveSelection_Mania"].Current.ToString()].Current == 255 || vars.watchers["Save_" + vars.watchers["SaveSelection_Mania"].Current.ToString()].Current == 0;
    } else {
        // First, check if you disabled autostart in Encore Mode
        if (!settings["sencore"])
            return false;

        // In encore mode, similar to mania mode, first we need to make sure our selection is not out of bounds
        if (vars.watchers["SaveSelection_Encore"].Current > 3)
            return false;

        // If the no-save option is selected, don't check the save and start immediately
        if (vars.watchers["SaveSelection_Encore"].Current == 0)
            return true;
        // In encore mode, the save file actually must be an empty one
        else
            return vars.watchers["Encore_Save_" + vars.watchers["SaveSelection_Encore"].Current.ToString()].Current;
    }
}

reset
{
    return
        // Automatically reset whenever you return to either the title screen or the main menu
        (settings["saveSelect"] && vars.watchers["LevelID"].Old != 1 && vars.watchers["LevelID"].Old != 2 && (vars.watchers["LevelID"].Current == 1 || vars.watchers["LevelID"].Current == 2))
        // Or, alternatively, whenever you open the dev menu, if you enabled the relative option in the settings
        || (settings["devMenu"] && vars.watchers["Status"].Changed && vars.watchers["Status"].Current == 8);
}
