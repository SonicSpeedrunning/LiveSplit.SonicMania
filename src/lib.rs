#![no_std]
#![feature(type_alias_impl_trait, const_async_blocks)]
#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::undocumented_unsafe_blocks,
    rust_2018_idioms
)]

use asr::{
    file_format::pe::{self, MachineType},
    future::{next_tick, retry},
    settings::Gui,
    signature::Signature,
    timer::{self, TimerState},
    watcher::Watcher,
    Address, Address32, Process,
};

asr::panic_handler!();
asr::async_main!(nightly);

const PROCESS_NAMES: &[&str] = &[
    "SonicMania.exe",
    "RSDKv5.exe",
    "RSDKv5-dx11.exe",
    "RSDKv5-dx9.exe",
    "RSDKv5U.exe",
    "RSDKv5U-dx9.exe",
    "RSDKv5U-dx11.exe",
    "RSDKv5_x64.exe",
    "RSDKv5-dx9_64.exe",
    "RSDKv5-dx11_64.exe",
    "RSDKv5U_x64.exe",
    "RSDKv5U-dx11_64.exe",
    "RSDKv5U-dx9_64.exe",
];

async fn main() {
    let mut settings = Settings::register();

    loop {
        // Hook to the target process
        let process = retry(|| PROCESS_NAMES.iter().find_map(|&name| Process::attach(name))).await;

        process
            .until_closes(async {
                // Once the target has been found and attached to, set up some default watchers
                let mut watchers = Watchers::default();

                // Perform memory scanning to look for the addresses we need
                let addresses = Addresses::init(&process).await;

                loop {
                    // Splitting logic. Adapted from OG LiveSplit:
                    // Order of execution
                    // 1. update() will always be run first. There are no conditions on the execution of this action.
                    // 2. If the timer is currently either running or paused, then the isLoading, gameTime, and reset actions will be run.
                    // 3. If reset does not return true, then the split action will be run.
                    // 4. If the timer is currently not running (and not paused), then the start action will be run.
                    settings.update();
                    update_loop(&process, &addresses, &mut watchers);

                    let timer_state = timer::state();
                    if timer_state == TimerState::Running || timer_state == TimerState::Paused {
                        if reset(&watchers, &settings) {
                            timer::reset()
                        } else if split(&watchers, &settings) {
                            timer::split()
                        }
                    }

                    if timer::state() == TimerState::NotRunning && start(&watchers, &settings) {
                        timer::start();
                    }

                    next_tick().await;
                }
            })
            .await;
    }
}

#[derive(Default)]
struct Watchers {
    level_id_numeric: Watcher<u8>,
    level_id: Watcher<Acts>,
    status: Watcher<u8>,
    egg_reverie_health: Watcher<u8>,
    titanic_monarch_2_defeated: Watcher<bool>,
    chaos_emeralds: Watcher<u8>,
    game_mode: Watcher<GameMode>,
    character_1: Watcher<u8>,
    character_2: Watcher<u8>,
    start_trigger: Watcher<u32>,
    mania_mode_selection: Watcher<bool>,
    save_selection_mania: Watcher<u8>,
    save_0: Watcher<u8>,
    save_1: Watcher<u8>,
    save_2: Watcher<u8>,
    save_3: Watcher<u8>,
    save_4: Watcher<u8>,
    save_5: Watcher<u8>,
    save_6: Watcher<u8>,
    save_7: Watcher<u8>,
    save_selection_encore: Watcher<u8>,
    save_encore_1: Watcher<bool>,
    save_encore_2: Watcher<bool>,
    save_encore_3: Watcher<bool>,
}

#[derive(Gui)]
struct Settings {
    #[default = true]
    /// START: Enable auto start in Mania mode
    start_mania_mode: bool,
    #[default = true]
    /// START: Enable auto start in Encore mode
    start_encore_mode: bool,
    #[default = true]
    /// RESET: Auto reset when returning to the save selection screen
    reset_save_select: bool,
    #[default = false]
    /// RESET: Auto reset when opening the dev menu
    reset_dev_menu: bool,
    #[default = true]
    /// Green Hill Act 1
    green_hill_1: bool,
    #[default = true]
    /// Green Hill Act 2
    green_hill_2: bool,
    #[default = true]
    /// Chemical Plant Act 1
    chemical_plant_1: bool,
    #[default = true]
    /// Chemical Plant Act 2
    chemical_plant_2: bool,
    #[default = true]
    /// Studiopolis Act 1
    studiopolis_1: bool,
    #[default = true]
    /// Studiopolis Act 2
    studiopolis_2: bool,
    #[default = true]
    /// Flying Battery Act 1
    flying_battery_1: bool,
    #[default = true]
    /// Flying Battery Act 2
    flying_battery_2: bool,
    #[default = true]
    /// Press Garden Act 1
    press_garden_1: bool,
    #[default = true]
    /// Press Garden Act 2
    press_garden_2: bool,
    #[default = true]
    /// Stardust Speedway Act 1
    stardust_speedway_1: bool,
    #[default = true]
    /// Stardust Speedway Act 2
    stardust_speedway_2: bool,
    #[default = true]
    /// Hydrocity Act 1
    hydrocity_1: bool,
    #[default = true]
    /// Hydrocity Act 2
    hydrocity_2: bool,
    #[default = true]
    /// Mirage Saloon Act 1
    mirage_saloon_1: bool,
    #[default = true]
    /// Mirage Saloon Act 2
    mirage_saloon_2: bool,
    #[default = true]
    /// Oil Ocean Act 1
    oil_ocean_1: bool,
    #[default = true]
    /// Oil Ocean Act 2
    oil_ocean_2: bool,
    #[default = true]
    /// Lava Reef Act 1
    lava_reef_1: bool,
    #[default = true]
    /// Lava Reef Act 2
    lava_reef_2: bool,
    #[default = true]
    /// Metallic Madness Act 1
    metallic_madness_1: bool,
    #[default = true]
    /// Metallic Madness Act 2
    metallic_madness_2: bool,
    #[default = true]
    /// Titanic Monarch Act 1
    titanic_monarch_1: bool,
    #[default = true]
    /// Titanic Monarch Act 2
    titanic_monarch_2: bool,
    #[default = true]
    /// Egg Reverie
    egg_reverie: bool,
    #[default = true]
    /// ENCORE: Angel Island
    angel_island: bool,
    #[default = true]
    /// ENCORE: Green Hill Act 1
    encore_green_hill_1: bool,
    #[default = true]
    /// ENCORE: Green Hill Act 2
    encore_green_hill_2: bool,
    #[default = true]
    /// ENCORE: Chemical Plant Act 1
    encore_chemical_plant_1: bool,
    #[default = true]
    /// ENCORE: Chemical Plant Act 2,
    encore_chemical_plant_2: bool,
    #[default = true]
    /// ENCORE: Studiopolis Act 1
    encore_studiopolis_1: bool,
    #[default = true]
    /// ENCORE: Studiopolis Act 2
    encore_studiopolis_2: bool,
    #[default = true]
    /// ENCORE: Flying Battery Act 1
    encore_flying_battery_1: bool,
    #[default = true]
    /// ENCORE: Flying Battery Act 2
    encore_flying_battery_2: bool,
    #[default = true]
    /// ENCORE: Press Garden Act 1
    encore_press_garden_1: bool,
    #[default = true]
    /// ENCORE: Press Garden Act 2
    encore_press_garden_2: bool,
    #[default = true]
    /// ENCORE: Stardust Speedway Act 1
    encore_stardust_speedway_1: bool,
    #[default = true]
    /// ENCORE: Stardust Speedway Act 2
    encore_stardust_speedway_2: bool,
    #[default = true]
    /// ENCORE: Hydrocity Act 1
    encore_hydrocity_1: bool,
    #[default = true]
    /// ENCORE: Hydrocity Act 2
    encore_hydrocity_2: bool,
    #[default = true]
    /// ENCORE: Mirage Saloon Act 1
    encore_mirage_saloon_1: bool,
    #[default = true]
    /// ENCORE: Mirage Saloon Act 2
    encore_mirage_saloon_2: bool,
    #[default = true]
    /// ENCORE: Oil Ocean Act 1
    encore_oil_ocean_1: bool,
    #[default = true]
    /// ENCORE: Oil Ocean Act 2
    encore_oil_ocean_2: bool,
    #[default = true]
    /// ENCORE: Lava Reef Act 1
    encore_lava_reef_1: bool,
    #[default = true]
    /// ENCORE: Lava Reef Act 2
    encore_lava_reef_2: bool,
    #[default = true]
    /// ENCORE: Metallic Madness Act 1
    encore_metallic_madness_1: bool,
    #[default = true]
    /// ENCORE: Metallic Madness Act 2
    encore_metallic_madness_2: bool,
    #[default = true]
    /// ENCORE: Titanic Monarch Act 1
    encore_titanic_monarch_1: bool,
    #[default = true]
    /// ENCORE: Titanic Monarch Act 2
    encore_titanic_monarch_2: bool,
}

struct Addresses {
    is_64_bit: bool,
    level_id_base: Address,
    level_id_offset: u32,
    status_base: Address,
    status_offset: u32,
    egg_reverie_base: Address,
    egg_reverie1_offset1: u32,
    egg_reverie1_offset2: u32,
    egg_reverie2_offset1: u32,
    egg_reverie2_offset2: u32,
    egg_monarch_base: Address,
    egg_monarch_offset: u32,
    chaos_emerald_base: Address,
    chaos_emerald_offset1: u32,
    chaos_emerald_offset2: u32,
    character_base: Address,
    character_offset: u32,
    start_trigger: Address,
    mania_mode_selection: Address,
    save_selection_mania: Address,
    save_0: Address,
    save_1: Address,
    save_2: Address,
    save_3: Address,
    save_4: Address,
    save_5: Address,
    save_6: Address,
    save_7: Address,
    save_selection_encore: Address,
    encore_save_1: Address,
    encore_save_2: Address,
    encore_save_3: Address,
}

impl Addresses {
    async fn init(process: &Process) -> Self {
        let main_module = {
            let main_module_base = retry(|| {
                PROCESS_NAMES
                    .iter()
                    .find_map(|&name| process.get_module_address(name).ok())
            })
            .await;
            let main_module_size =
                retry(|| pe::read_size_of_image(process, main_module_base)).await as u64;
            (main_module_base, main_module_size)
        };

        let is_64_bit =
            retry(|| pe::MachineType::read(process, main_module.0)).await == MachineType::X86_64;

        let base: Address;
        let size: u64;
        let mut ptr: Address;

        // Try to find the Game.dll module. If it succeeds, it means we are running on the decompilation
        if let Ok(ubase) = process.get_module_address("Game.dll") {
            // In the decomp, most of the sigscanning will be performed in the Game.dll module
            base = ubase;
            size = retry(|| pe::read_size_of_image(process, base)).await as _;
        } else {
            // In the retail version, we need only the main module, which means base and size are
            // going to be equal the main_module_base and main_module_size.
            base = main_module.0;
            size = main_module.1;
        }

        let is_rsdk_5u: bool;
        let level_id_base: Address;
        let level_id_offset: u32;
        let status_base: Address;
        let status_offset: u32;
        let egg_reverie_base: Address;
        let egg_reverie1_offset1: u32;
        let egg_reverie1_offset2: u32;
        let egg_reverie2_offset1: u32;
        let egg_reverie2_offset2: u32;
        let egg_monarch_base: Address;
        let egg_monarch_offset: u32;
        let chaos_emerald_base: Address;
        let chaos_emerald_offset1: u32;
        let chaos_emerald_offset2: u32;
        let character_base: Address;
        let character_offset: u32;
        let start_trigger: Address;
        let mania_mode_selection: Address;
        let save_selection_mania: Address;
        let save_0: Address;
        let save_1: Address;
        let save_2: Address;
        let save_3: Address;
        let save_4: Address;
        let save_5: Address;
        let save_6: Address;
        let save_7: Address;
        let save_selection_encore: Address;
        let encore_save_1: Address;
        let encore_save_2: Address;
        let encore_save_3: Address;

        // 32-bit version
        if !is_64_bit {
            // Detect if we're on rsdk5U or not. It alters some of the offsets later.
            // The retail version of the game is NEVER rsdk5U
            const SIGRSDK5U: Signature<24> =
                Signature::new("3D ???????? 0F 87 ???????? FF 24 85 ???????? A1 ???????? 89");
            is_rsdk_5u = SIGRSDK5U.scan_process_range(process, main_module).is_some();

            // LevelID. Actually a SceneID, it also identifies whenever we are in the main menu
            const SIG: Signature<14> = Signature::new("8B ?? ???????? 85 C9 74 1E 80 ?? ?? 03");
            ptr = retry(|| SIG.scan_process_range(process, (base, size))).await + 2;
            level_id_base = retry(|| process.read::<Address32>(ptr)).await.into();
            level_id_offset = retry(|| process.read::<u8>(ptr + 28)).await as u32;

            // Status and in-game timer. Doesn't even need an explanation, though we are not gonna use it directly
            const SIGSTATUS: Signature<16> =
                Signature::new("A1 ???????? 80 78 ?? 01 0F 85 ???????? FF");
            ptr = retry(|| SIGSTATUS.scan_process_range(process, (base, size))).await + 1;
            status_base = retry(|| process.read::<Address32>(ptr)).await.into();
            status_offset = retry(|| process.read::<u8>(ptr + 6)).await as u32;

            // Health addresses for the bosses in Egg Reverie
            const SIG_ER: Signature<13> = Signature::new("8B ?? ???????? 8B ?? 40 83 ?? ?? 00");
            ptr = retry(|| SIG_ER.scan_process_range(process, (base, size))).await + 2;
            egg_reverie_base = retry(|| process.read::<Address32>(ptr)).await.into();
            egg_reverie1_offset1 = retry(|| process.read::<u8>(ptr + 6)).await as _;
            egg_reverie1_offset2 = retry(|| process.read::<u8>(ptr + 9)).await as _;
            egg_reverie2_offset1 = retry(|| process.read::<u8>(ptr + 15)).await as _;
            egg_reverie2_offset2 = retry(|| process.read::<u8>(ptr + 18)).await as _;

            // Flag that immediately tells when we defeated Eggman in Titanic Monarch Act 2
            const SIG_EM1: Signature<8> = Signature::new("A1 ???????? 83 C0 78");
            ptr = retry(|| SIG_EM1.scan_process_range(process, (base, size))).await + 1;
            egg_monarch_base = retry(|| process.read::<Address32>(ptr)).await.into();
            const SIG_EM2: Signature<24> =
                Signature::new("6A 00 C7 80 ???????? 01 00 00 00 A1 ???????? C7 ?? ?? 00 00 00 00");
            ptr = retry(|| SIG_EM2.scan_process_range(process, (base, size))).await + 4;
            egg_monarch_offset = retry(|| process.read::<u32>(ptr)).await;

            // Bitmask for the Chaos Emeralds. One bit for every emerald. 0x7F = 7 emeralds.
            const SIG_CE: Signature<14> = Signature::new("7D ?? A1 ???????? 8B ?? ?? 83 ?? ?? 7F");
            ptr = retry(|| SIG_CE.scan_process_range(process, (base, size))).await + 3;
            chaos_emerald_base = retry(|| process.read::<Address32>(ptr)).await.into();
            chaos_emerald_offset1 = retry(|| process.read::<u8>(ptr + 6)).await as _;
            chaos_emerald_offset2 = retry(|| process.read::<u8>(ptr + 9)).await as _;

            // Characters
            const SIG_CHARACTER: Signature<10> = Signature::new("8B 15 ???????? 8B 4A 04 C1");
            ptr = retry(|| SIG_CHARACTER.scan_process_range(process, (base, size))).await + 2;
            character_base = retry(|| process.read::<Address32>(ptr)).await.into();
            character_offset = retry(|| process.read::<u8>(ptr + 6)).await as _;

            // The start trigger monitors whenever the game reports a successful selection of the save file
            // Very janky solution, but it works.
            const SIG_SAVE: Signature<14> = Signature::new("69 C0 ???????? 05 ???????? 5D C3 CC");
            ptr = retry(|| SIG_SAVE.scan_process_range(process, main_module)).await + 7;
            ptr = retry(|| process.read::<Address32>(ptr)).await.into();
            start_trigger = ptr + 0x71808 + if is_rsdk_5u { 0xD14 } else { 0 };
            mania_mode_selection = ptr + 0x720AC + if is_rsdk_5u { 0xD24 } else { 0 };
            save_selection_mania = ptr + 0x720C0 + if is_rsdk_5u { 0xD24 } else { 0 };
            save_0 = ptr + 0x146DC + if is_rsdk_5u { 0x25C } else { 0 };
            save_1 = ptr + 0x146DC + if is_rsdk_5u { 0x25C + 0x460 } else { 0x458 };
            save_2 = ptr
                + 0x146DC
                + if is_rsdk_5u {
                    0x25C + 0x460 * 2
                } else {
                    0x458 * 2
                };
            save_3 = ptr
                + 0x146DC
                + if is_rsdk_5u {
                    0x25C + 0x460 * 3
                } else {
                    0x458 * 3
                };
            save_4 = ptr
                + 0x146DC
                + if is_rsdk_5u {
                    0x25C + 0x460 * 4
                } else {
                    0x458 * 4
                };
            save_5 = ptr
                + 0x146DC
                + if is_rsdk_5u {
                    0x25C + 0x460 * 5
                } else {
                    0x458 * 5
                };
            save_6 = ptr
                + 0x146DC
                + if is_rsdk_5u {
                    0x25C + 0x460 * 6
                } else {
                    0x458 * 6
                };
            save_7 = ptr
                + 0x146DC
                + if is_rsdk_5u {
                    0x25C + 0x460 * 7
                } else {
                    0x458 * 7
                };

            save_selection_encore = ptr + 0x7A768 + if is_rsdk_5u { 0xE1C } else { 0 };
            encore_save_1 = ptr + 0x6F5F4 + if is_rsdk_5u { 0xCD4 + 0x460 } else { 0x458 };
            encore_save_2 = ptr
                + 0x6F5F4
                + if is_rsdk_5u {
                    0xCD4 + 0x460 * 2
                } else {
                    0x458 * 2
                };
            encore_save_3 = ptr
                + 0x6F5F4
                + if is_rsdk_5u {
                    0xCD4 + 0x460 * 3
                } else {
                    0x458 * 3
                };
        } else {
            // 64-bit version (only for decomps)
            // Detect if we're on rsdk5U or not. It alters some of the offsets later.
            const SIGRSDK5U: Signature<15> =
                Signature::new("81 F9 ???????? 0F 87 ???????? 41 8B 8C");
            is_rsdk_5u = SIGRSDK5U.scan_process_range(process, main_module).is_some();

            // LevelID. Actually a SceneID, it also identifies whenever we are in the main menu
            const SIG: Signature<14> = Signature::new("48 8B ?? ???????? 48 85 C9 74 2A 80 7A");
            ptr = retry(|| SIG.scan_process_range(process, (base, size))).await + 3;
            level_id_base = ptr + 0x4 + retry(|| process.read::<i32>(ptr)).await;
            level_id_offset = retry(|| process.read::<u8>(ptr + 30)).await as _;

            // In-game timer. Doesn't even need an explanation, though we are not gonna use it directly
            const SIGSTATUS: Signature<18> =
                Signature::new("48 8B ?? ???????? 80 78 ?? 01 0F 85 ???????? FF");
            ptr = retry(|| SIGSTATUS.scan_process_range(process, (base, size))).await + 3;
            status_base = ptr + 0x4 + retry(|| process.read::<i32>(ptr)).await;
            status_offset = retry(|| process.read::<u8>(ptr + 6)).await as _;

            // Health addresses for the bosses in Egg Reverie
            const SIG_ER: Signature<14> = Signature::new("75 ?? 48 8B 0D ???????? 48 8B 41 ?? 83");
            ptr = retry(|| SIG_ER.scan_process_range(process, (base, size))).await + 5;
            egg_reverie_base = ptr + 0x4 + retry(|| process.read::<i32>(ptr)).await;
            egg_reverie1_offset1 = retry(|| process.read::<u8>(ptr + 7)).await as _;
            egg_reverie1_offset2 = retry(|| process.read::<u8>(ptr + 10)).await as _;
            const SIG_ER_2: Signature<14> =
                Signature::new("75 ?? 48 8B 0D ???????? 48 8B 41 ?? 83");
            ptr = retry(|| SIG_ER_2.scan_process_range(process, (base, size))).await;
            egg_reverie2_offset1 = retry(|| process.read::<u8>(ptr + 5)).await as _;
            egg_reverie2_offset2 = retry(|| process.read::<u8>(ptr + 8)).await as _;

            // Flag that immediately tells when we defeated Eggman in Titanic Monarch Act 2
            const SIG_EM: Signature<20> =
                Signature::new("89 73 ?? 48 8B ?? ???????? C7 ?? ???????? 01 00 00 00");
            ptr = retry(|| SIG_EM.scan_process_range(process, (base, size))).await + 6;
            egg_monarch_base = ptr + 0x4 + retry(|| process.read::<i32>(ptr)).await;
            egg_monarch_offset = retry(|| process.read::<u32>(ptr + 6)).await as _;

            // Bitmask for the Chaos Emeralds. One bit for every emerald. 0x7F = 7 emeralds.
            const SIG_CE: Signature<18> =
                Signature::new("7D ?? 48 8B ?? ???????? 48 ?? ?? ?? 83 ?? ?? 7F 7C");
            ptr = retry(|| SIG_CE.scan_process_range(process, (base, size))).await + 5;
            chaos_emerald_base = ptr + 0x4 + retry(|| process.read::<i32>(ptr)).await;
            chaos_emerald_offset1 = retry(|| process.read::<u8>(ptr + 7)).await as _;
            chaos_emerald_offset2 = retry(|| process.read::<u8>(ptr + 10)).await as _;

            // Characters
            const SIG_CHARACTER: Signature<13> =
                Signature::new("48 8B ?? ???????? 8B 4A ?? C1 F9 08");
            ptr = retry(|| SIG_CHARACTER.scan_process_range(process, (base, size))).await + 3;
            character_base = ptr + 0x4 + retry(|| process.read::<i32>(ptr)).await;
            character_offset = retry(|| process.read::<u8>(ptr + 6)).await as _;

            // The start trigger monitors whenever the game reports a successful selection of the save file
            // Very janky solution, but it works.
            const SIG_SAVE: Signature<18> =
                Signature::new("4C 8D ?? ???????? 8B D0 4C 8D ?? ???????? 0F 1F");
            ptr = retry(|| SIG_SAVE.scan_process_range(process, main_module)).await + 3;
            ptr = ptr + retry(|| process.read::<i32>(ptr)).await + 0x4;
            start_trigger = ptr + 0xD977A + if is_rsdk_5u { 0x1A10 } else { 0 };
            mania_mode_selection = ptr + 0xDA81E + if is_rsdk_5u { 0x1A30 } else { 0 };
            save_selection_mania = ptr + 0xDA836 + if is_rsdk_5u { 0x1A30 } else { 0 };
            save_0 = ptr + 0x26A9A + if is_rsdk_5u { 0x4A0 } else { 0 };
            save_1 = ptr + 0x26A9A + if is_rsdk_5u { 0x4A0 + 0x868 } else { 0x858 };
            save_2 = ptr
                + 0x26A9A
                + if is_rsdk_5u {
                    0x4A0 + 0x868 * 2
                } else {
                    0x858 * 2
                };
            save_3 = ptr
                + 0x26A9A
                + if is_rsdk_5u {
                    0x4A0 + 0x868 * 3
                } else {
                    0x858 * 3
                };
            save_4 = ptr
                + 0x26A9A
                + if is_rsdk_5u {
                    0x4A0 + 0x868 * 4
                } else {
                    0x858 * 4
                };
            save_5 = ptr
                + 0x26A9A
                + if is_rsdk_5u {
                    0x4A0 + 0x868 * 5
                } else {
                    0x858 * 5
                };
            save_6 = ptr
                + 0x26A9A
                + if is_rsdk_5u {
                    0x4A0 + 0x868 * 6
                } else {
                    0x858 * 6
                };
            save_7 = ptr
                + 0x26A9A
                + if is_rsdk_5u {
                    0x4A0 + 0x868 * 7
                } else {
                    0x858 * 7
                };

            save_selection_encore = ptr + 0xEAADE + if is_rsdk_5u { 0x1C20 } else { 0 };
            encore_save_1 = ptr + 0xD55AA + if is_rsdk_5u { 0x1990 + 0x868 } else { 0x858 };
            encore_save_2 = ptr
                + 0xD55AA
                + if is_rsdk_5u {
                    0x1990 + 0x868 * 2
                } else {
                    0x858 * 2
                };
            encore_save_3 = ptr
                + 0xD55AA
                + if is_rsdk_5u {
                    0x1990 + 0x868 * 3
                } else {
                    0x858 * 3
                };
        }

        Self {
            is_64_bit,
            level_id_base,
            level_id_offset,
            status_base,
            status_offset,
            egg_reverie_base,
            egg_reverie1_offset1,
            egg_reverie1_offset2,
            egg_reverie2_offset1,
            egg_reverie2_offset2,
            egg_monarch_base,
            egg_monarch_offset,
            chaos_emerald_base,
            chaos_emerald_offset1,
            chaos_emerald_offset2,
            character_base,
            character_offset,
            start_trigger,
            mania_mode_selection,
            save_selection_mania,
            save_0,
            save_1,
            save_2,
            save_3,
            save_4,
            save_5,
            save_6,
            save_7,
            save_selection_encore,
            encore_save_1,
            encore_save_2,
            encore_save_3,
        }
    }
}

fn update_loop(game: &Process, addresses: &Addresses, watchers: &mut Watchers) {
    let game_mode: u8;
    let levelid: u8;
    let status: [u8; 5];
    let mut enum_levelid: Acts;
    let egg_reverie_monarch_health: u8;
    let egg_reverie_eggman_health: u8;
    let tm2_defeat: u8;
    let chaos_emeralds: u8;
    let characters: [u8; 2];

    if addresses.is_64_bit {
        game_mode = game
            .read_pointer_path64(addresses.character_base, &[0, 0])
            .ok()
            .unwrap_or_default();
        levelid = game
            .read_pointer_path64(
                addresses.level_id_base,
                &[0, addresses.level_id_offset as u64],
            )
            .ok()
            .unwrap_or_default();
        status = game
            .read_pointer_path64(addresses.status_base, &[0, addresses.status_offset as u64])
            .ok()
            .unwrap_or_default();
        egg_reverie_monarch_health = game
            .read_pointer_path64(
                addresses.egg_reverie_base,
                &[
                    0,
                    addresses.egg_reverie1_offset1 as u64,
                    addresses.egg_reverie1_offset2 as u64,
                ],
            )
            .ok()
            .unwrap_or_default();
        egg_reverie_eggman_health = game
            .read_pointer_path64(
                addresses.egg_reverie_base,
                &[
                    0,
                    addresses.egg_reverie2_offset1 as u64,
                    addresses.egg_reverie2_offset2 as u64,
                ],
            )
            .ok()
            .unwrap_or_default();
        tm2_defeat = game
            .read_pointer_path64(
                addresses.egg_monarch_base,
                &[0, addresses.egg_monarch_offset as u64],
            )
            .ok()
            .unwrap_or_default();
        chaos_emeralds = game
            .read_pointer_path64(
                addresses.chaos_emerald_base,
                &[
                    0,
                    addresses.chaos_emerald_offset1 as u64,
                    addresses.chaos_emerald_offset2 as u64,
                ],
            )
            .ok()
            .unwrap_or_default();
        characters = game
            .read_pointer_path64(
                addresses.character_base,
                &[0, addresses.character_offset as u64],
            )
            .ok()
            .unwrap_or_default();
    } else {
        game_mode = game
            .read_pointer_path32(addresses.character_base, &[0, 0])
            .ok()
            .unwrap_or_default();
        levelid = game
            .read_pointer_path32(addresses.level_id_base, &[0, addresses.level_id_offset])
            .ok()
            .unwrap_or_default();
        status = game
            .read_pointer_path32(addresses.status_base, &[0, addresses.status_offset])
            .ok()
            .unwrap_or_default();
        egg_reverie_monarch_health = game
            .read_pointer_path32(
                addresses.egg_reverie_base,
                &[
                    0,
                    addresses.egg_reverie1_offset1,
                    addresses.egg_reverie1_offset2,
                ],
            )
            .ok()
            .unwrap_or_default();
        egg_reverie_eggman_health = game
            .read_pointer_path32(
                addresses.egg_reverie_base,
                &[
                    0,
                    addresses.egg_reverie2_offset1,
                    addresses.egg_reverie2_offset2,
                ],
            )
            .ok()
            .unwrap_or_default();
        tm2_defeat = game
            .read_pointer_path32(
                addresses.egg_monarch_base,
                &[0, addresses.egg_monarch_offset],
            )
            .ok()
            .unwrap_or_default();
        chaos_emeralds = game
            .read_pointer_path32(
                addresses.chaos_emerald_base,
                &[
                    0,
                    addresses.chaos_emerald_offset1,
                    addresses.chaos_emerald_offset2,
                ],
            )
            .ok()
            .unwrap_or_default();
        characters = game
            .read_pointer_path32(addresses.character_base, &[0, addresses.character_offset])
            .ok()
            .unwrap_or_default();
    }

    // Level ID logic
    enum_levelid = match &watchers.level_id.pair {
        Some(lvl) => lvl.current,
        _ => Acts::GreenHill1,
    };

    // If level ID == 37, it's always Egg Reverie. Don't even consider the IGT because it's scrambled in that stage
    if levelid == 37 {
        enum_levelid = Acts::EggReverie;
    } else if status[0] == 2 || (status[0] != 0 && status[2] + status[3] + status[4] == 0) {
        enum_levelid = match game_mode {
            0 => match levelid {
                9 | 117 | 119 => Acts::GreenHill1,
                10 | 120 => Acts::GreenHill2,
                11 => Acts::ChemicalPlant1,
                12 => Acts::ChemicalPlant2,
                13 => Acts::Studiopolis1,
                14 => Acts::Studiopolis2,
                15 => Acts::FlyingBattery1,
                16 => Acts::FlyingBattery2,
                17 => Acts::PressGarden1,
                18 => Acts::PressGarden2,
                19 | 122 => Acts::StardustSpeedway1,
                20 | 21 => Acts::StardustSpeedway2,
                22 => Acts::Hydrocity1,
                23 => Acts::Hydrocity2,
                24 | 25 => Acts::MirageSaloon1,
                26 => Acts::MirageSaloon2,
                27 => Acts::OilOcean1,
                28 => Acts::OilOcean2,
                29 => Acts::LavaReef1,
                30 | 31 => Acts::LavaReef2,
                32 => Acts::MetallicMadness1,
                33 => Acts::MetallicMadness2,
                34 => Acts::TitanicMonarch1,
                35 | 36 => Acts::TitanicMonarch2,
                37 => Acts::EggReverie,
                _ => enum_levelid,
            },
            _ => match levelid {
                118 => Acts::EncoreAngelIsland,
                119 | 38 => Acts::EncoreGreenHill1,
                39 | 120 => Acts::EncoreGreenHill2,
                40 => Acts::EncoreChemicalPlant1,
                41 => Acts::EncoreChemicalPlant2,
                42 => Acts::EncoreStudiopolis1,
                43 => Acts::EncoreStudiopolis2,
                44 => Acts::EncoreFlyingBattery1,
                45 => Acts::EncoreFlyingBattery2,
                46 => Acts::EncorePressGarden1,
                47 => Acts::EncorePressGarden2,
                48 | 122 => Acts::EncoreStardustSpeedway1,
                49 | 50 => Acts::EncoreStardustSpeedway2,
                51 => Acts::EncoreHydrocity1,
                52 => Acts::EncoreHydrocity2,
                53 => Acts::EncoreMirageSaloon1,
                54 => Acts::EncoreMirageSaloon2,
                55 => Acts::EncoreOilOcean1,
                56 => Acts::EncoreOilOcean2,
                57 => Acts::EncoreLavaReef1,
                58 | 59 => Acts::EncoreLavaReef2,
                60 => Acts::EncoreMetallicMadness1,
                61 => Acts::EncoreMetallicMadness2,
                62 => Acts::EncoreTitanicMonarch1,
                63 | 64 => Acts::EncoreTitanicMonarch2,
                _ => enum_levelid,
            },
        };
    }
    watchers.level_id.update(Some(enum_levelid));

    watchers.game_mode.update(Some(if game_mode == 0 {
        GameMode::Standard
    } else if game_mode == 1 {
        GameMode::Encore
    } else {
        GameMode::Invalid
    }));
    watchers.level_id_numeric.update(Some(levelid));
    watchers.status.update(Some(status[0]));
    watchers
        .egg_reverie_health
        .update(Some(egg_reverie_eggman_health + egg_reverie_monarch_health));
    watchers
        .titanic_monarch_2_defeated
        .update(Some(tm2_defeat != 0));
    watchers.chaos_emeralds.update(Some(chaos_emeralds));
    watchers.character_1.update(Some(characters[0]));
    watchers.character_2.update(Some(characters[1]));

    watchers.start_trigger.update(Some(
        game.read(addresses.start_trigger).ok().unwrap_or_default(),
    ));
    watchers.mania_mode_selection.update(Some(
        game.read::<u8>(addresses.mania_mode_selection)
            .ok()
            .unwrap_or_default()
            != 0,
    ));
    watchers.save_selection_mania.update(Some(
        game.read(addresses.save_selection_mania)
            .ok()
            .unwrap_or_default(),
    ));
    watchers
        .save_0
        .update(Some(game.read(addresses.save_0).ok().unwrap_or_default()));
    watchers
        .save_1
        .update(Some(game.read(addresses.save_1).ok().unwrap_or_default()));
    watchers
        .save_2
        .update(Some(game.read(addresses.save_2).ok().unwrap_or_default()));
    watchers
        .save_3
        .update(Some(game.read(addresses.save_3).ok().unwrap_or_default()));
    watchers
        .save_4
        .update(Some(game.read(addresses.save_4).ok().unwrap_or_default()));
    watchers
        .save_5
        .update(Some(game.read(addresses.save_5).ok().unwrap_or_default()));
    watchers
        .save_6
        .update(Some(game.read(addresses.save_6).ok().unwrap_or_default()));
    watchers
        .save_7
        .update(Some(game.read(addresses.save_7).ok().unwrap_or_default()));
    watchers.save_selection_encore.update(Some(
        game.read(addresses.save_selection_encore)
            .ok()
            .unwrap_or_default(),
    ));
    watchers.save_encore_1.update(Some(
        game.read::<u8>(addresses.encore_save_1)
            .ok()
            .unwrap_or_default()
            != 0,
    ));
    watchers.save_encore_2.update(Some(
        game.read::<u8>(addresses.encore_save_2)
            .ok()
            .unwrap_or_default()
            != 0,
    ));
    watchers.save_encore_3.update(Some(
        game.read::<u8>(addresses.encore_save_3)
            .ok()
            .unwrap_or_default()
            != 0,
    ));
}

fn start(watchers: &Watchers, settings: &Settings) -> bool {
    if !settings.start_mania_mode && !settings.start_encore_mode {
        return false;
    }

    // If you're not in the main menu, there's no reason to continue
    let Some(levelid) = &watchers.level_id_numeric.pair else {
        return false;
    };
    if levelid.current != 2 {
        return false;
    };

    // If the main trigger doesn't get involved, there's no reason to continue
    let Some(start_trigger) = &watchers.start_trigger.pair else {
        return false;
    };
    if start_trigger.current != start_trigger.old + 48 {
        return false;
    };

    let Some(mania_mode_selection) = &watchers.mania_mode_selection.pair else {
        return false;
    };
    if mania_mode_selection.current {
        if !settings.start_mania_mode {
            return false;
        };

        // If, for whatever reason, the save selection is out of bounds, do not continue
        let Some(save_selection_mania) = &watchers.save_selection_mania.pair else {
            return false;
        };
        if save_selection_mania.current > 8 {
            return false;
        };

        // If the no-save option is selected, there's no need to check the save files
        if save_selection_mania.current == 8 {
            true
        } else {
            let Some(save_0) = &watchers.save_0.pair else {
                return false;
            };
            let Some(save_1) = &watchers.save_1.pair else {
                return false;
            };
            let Some(save_2) = &watchers.save_2.pair else {
                return false;
            };
            let Some(save_3) = &watchers.save_3.pair else {
                return false;
            };
            let Some(save_4) = &watchers.save_4.pair else {
                return false;
            };
            let Some(save_5) = &watchers.save_5.pair else {
                return false;
            };
            let Some(save_6) = &watchers.save_6.pair else {
                return false;
            };
            let Some(save_7) = &watchers.save_7.pair else {
                return false;
            };

            // This essentially checks if you selected an empty save file (green hill zone 1). Works for both new games and new game +
            match save_selection_mania.current {
                0 => save_0.current == 255 || save_0.current == 0,
                1 => save_1.current == 255 || save_1.current == 0,
                2 => save_2.current == 255 || save_2.current == 0,
                3 => save_3.current == 255 || save_3.current == 0,
                4 => save_4.current == 255 || save_4.current == 0,
                5 => save_5.current == 255 || save_5.current == 0,
                6 => save_6.current == 255 || save_6.current == 0,
                7 => save_7.current == 255 || save_7.current == 0,
                _ => false,
            }
        }
    } else {
        if !settings.start_encore_mode {
            return false;
        };

        let Some(save_selection_encore) = &watchers.save_selection_encore.pair else {
            return false;
        };
        if save_selection_encore.current > 3 {
            return false;
        };
        if save_selection_encore.current == 0 {
            true
        } else {
            let Some(save_1) = &watchers.save_encore_1.pair else {
                return false;
            };
            let Some(save_2) = &watchers.save_encore_2.pair else {
                return false;
            };
            let Some(save_3) = &watchers.save_encore_3.pair else {
                return false;
            };
            match save_selection_encore.current {
                1 => save_1.current,
                2 => save_2.current,
                3 => save_3.current,
                _ => false,
            }
        }
    }
}

fn split(watchers: &Watchers, settings: &Settings) -> bool {
    let Some(game_mode) = &watchers.game_mode.pair else {
        return false;
    };
    let Some(level_id) = &watchers.level_id.pair else {
        return false;
    };

    // If you're outside Mania or Encore mode, there's no reason to continue
    if game_mode.current != GameMode::Standard && game_mode.current != GameMode::Encore {
        return false;
    };

    if level_id.old == Acts::TitanicMonarch2 || level_id.old == Acts::EncoreTitanicMonarch2 {
        if game_mode.current == GameMode::Standard {
            if settings.titanic_monarch_2 {
                let Some(chaos_emeralds) = &watchers.chaos_emeralds.pair else {
                    return false;
                };
                let Some(character_1) = &watchers.character_1.pair else {
                    return false;
                };
                let Some(character_2) = &watchers.character_2.pair else {
                    return false;
                };

                if chaos_emeralds.current == 0x7F
                    && (character_1.current == 1
                        || (character_1.current == 4 && character_2.current == 4))
                {
                    return level_id.current == Acts::EggReverie;
                } else {
                    let Some(tm2_defeated) = &watchers.titanic_monarch_2_defeated.pair else {
                        return false;
                    };
                    return tm2_defeated.current && !tm2_defeated.old;
                }
            }
        } else {
            let Some(tm2_defeated) = &watchers.titanic_monarch_2_defeated.pair else {
                return false;
            };
            return settings.encore_titanic_monarch_2 && tm2_defeated.current && !tm2_defeated.old;
        }
    }
    // Egg Reverie needs a split when the bosses are defeated
    else if level_id.old == Acts::EggReverie {
        if settings.egg_reverie {
            let Some(status) = &watchers.status.pair else {
                return false;
            };
            let Some(er_health) = &watchers.egg_reverie_health.pair else {
                return false;
            };
            return status.current == 1 && er_health.old > 0 && er_health.current == 0;
        }
    } else {
        return match level_id.old {
            Acts::GreenHill1 => settings.green_hill_1 && level_id.current == Acts::GreenHill2,
            Acts::GreenHill2 => settings.green_hill_2 && level_id.current == Acts::ChemicalPlant1,
            Acts::ChemicalPlant1 => {
                settings.chemical_plant_1 && level_id.current == Acts::ChemicalPlant2
            }
            Acts::ChemicalPlant2 => {
                settings.chemical_plant_2 && level_id.current == Acts::Studiopolis1
            }
            Acts::Studiopolis1 => settings.studiopolis_1 && level_id.current == Acts::Studiopolis2,
            Acts::Studiopolis2 => {
                settings.studiopolis_2 && level_id.current == Acts::FlyingBattery1
            }
            Acts::FlyingBattery1 => {
                settings.flying_battery_1 && level_id.current == Acts::FlyingBattery2
            }
            Acts::FlyingBattery2 => {
                settings.flying_battery_2 && level_id.current == Acts::PressGarden1
            }
            Acts::PressGarden1 => settings.press_garden_1 && level_id.current == Acts::PressGarden2,
            Acts::PressGarden2 => {
                settings.press_garden_2 && level_id.current == Acts::StardustSpeedway1
            }
            Acts::StardustSpeedway1 => {
                settings.stardust_speedway_1 && level_id.current == Acts::StardustSpeedway2
            }
            Acts::StardustSpeedway2 => {
                settings.stardust_speedway_2 && level_id.current == Acts::Hydrocity1
            }
            Acts::Hydrocity1 => settings.hydrocity_1 && level_id.current == Acts::Hydrocity2,
            Acts::Hydrocity2 => settings.hydrocity_2 && level_id.current == Acts::MirageSaloon1,
            Acts::MirageSaloon1 => {
                settings.mirage_saloon_1 && level_id.current == Acts::MirageSaloon2
            }
            Acts::MirageSaloon2 => settings.mirage_saloon_2 && level_id.current == Acts::OilOcean1,
            Acts::OilOcean1 => settings.oil_ocean_1 && level_id.current == Acts::OilOcean2,
            Acts::OilOcean2 => settings.oil_ocean_2 && level_id.current == Acts::LavaReef1,
            Acts::LavaReef1 => settings.lava_reef_1 && level_id.current == Acts::LavaReef2,
            Acts::LavaReef2 => settings.lava_reef_2 && level_id.current == Acts::MetallicMadness1,
            Acts::MetallicMadness1 => {
                settings.metallic_madness_1 && level_id.current == Acts::MetallicMadness2
            }
            Acts::MetallicMadness2 => {
                settings.metallic_madness_2 && level_id.current == Acts::TitanicMonarch1
            }
            Acts::TitanicMonarch1 => {
                settings.titanic_monarch_1 && level_id.current == Acts::TitanicMonarch2
            }
            Acts::EncoreAngelIsland => {
                settings.angel_island && level_id.current == Acts::EncoreGreenHill1
            }
            Acts::EncoreGreenHill1 => {
                settings.encore_green_hill_1 && level_id.current == Acts::EncoreGreenHill2
            }
            Acts::EncoreGreenHill2 => {
                settings.encore_green_hill_2 && level_id.current == Acts::EncoreChemicalPlant1
            }
            Acts::EncoreChemicalPlant1 => {
                settings.encore_chemical_plant_1 && level_id.current == Acts::EncoreChemicalPlant2
            }
            Acts::EncoreChemicalPlant2 => {
                settings.encore_chemical_plant_2 && level_id.current == Acts::EncoreStudiopolis1
            }
            Acts::EncoreStudiopolis1 => {
                settings.encore_studiopolis_1 && level_id.current == Acts::EncoreStudiopolis2
            }
            Acts::EncoreStudiopolis2 => {
                settings.encore_studiopolis_2 && level_id.current == Acts::EncoreFlyingBattery1
            }
            Acts::EncoreFlyingBattery1 => {
                settings.encore_flying_battery_1 && level_id.current == Acts::EncoreFlyingBattery2
            }
            Acts::EncoreFlyingBattery2 => {
                settings.encore_flying_battery_2 && level_id.current == Acts::EncorePressGarden1
            }
            Acts::EncorePressGarden1 => {
                settings.encore_press_garden_1 && level_id.current == Acts::EncorePressGarden2
            }
            Acts::EncorePressGarden2 => {
                settings.encore_press_garden_2 && level_id.current == Acts::EncoreStardustSpeedway1
            }
            Acts::EncoreStardustSpeedway1 => {
                settings.encore_stardust_speedway_1
                    && level_id.current == Acts::EncoreStardustSpeedway2
            }
            Acts::EncoreStardustSpeedway2 => {
                settings.encore_stardust_speedway_2 && level_id.current == Acts::EncoreHydrocity1
            }
            Acts::EncoreHydrocity1 => {
                settings.encore_hydrocity_1 && level_id.current == Acts::EncoreHydrocity2
            }
            Acts::EncoreHydrocity2 => {
                settings.encore_hydrocity_2 && level_id.current == Acts::EncoreMirageSaloon1
            }
            Acts::EncoreMirageSaloon1 => {
                settings.encore_mirage_saloon_1 && level_id.current == Acts::EncoreMirageSaloon2
            }
            Acts::EncoreMirageSaloon2 => {
                settings.encore_mirage_saloon_2 && level_id.current == Acts::EncoreOilOcean1
            }
            Acts::EncoreOilOcean1 => {
                settings.encore_oil_ocean_1 && level_id.current == Acts::EncoreOilOcean2
            }
            Acts::EncoreOilOcean2 => {
                settings.encore_oil_ocean_2 && level_id.current == Acts::EncoreLavaReef1
            }
            Acts::EncoreLavaReef1 => {
                settings.encore_lava_reef_1 && level_id.current == Acts::EncoreLavaReef2
            }
            Acts::EncoreLavaReef2 => {
                settings.encore_lava_reef_2 && level_id.current == Acts::EncoreMetallicMadness1
            }
            Acts::EncoreMetallicMadness1 => {
                settings.encore_metallic_madness_1
                    && level_id.current == Acts::EncoreMetallicMadness2
            }
            Acts::EncoreMetallicMadness2 => {
                settings.encore_metallic_madness_2
                    && level_id.current == Acts::EncoreTitanicMonarch1
            }
            Acts::EncoreTitanicMonarch1 => {
                settings.encore_titanic_monarch_1 && level_id.current == Acts::EncoreTitanicMonarch2
            }
            _ => false,
        };
    }
    false
}

fn reset(watchers: &Watchers, settings: &Settings) -> bool {
    let Some(status) = &watchers.status.pair else {
        return false;
    };
    let Some(levelid_numeric) = &watchers.level_id_numeric.pair else {
        return false;
    };

    (settings.reset_save_select
        && levelid_numeric.old != 1
        && levelid_numeric.old != 2
        && (levelid_numeric.current == 1 || levelid_numeric.current == 2))
        || (settings.reset_dev_menu && status.changed() && status.current == 8)
}

#[derive(Clone, Copy, PartialEq)]
enum Acts {
    GreenHill1,
    GreenHill2,
    ChemicalPlant1,
    ChemicalPlant2,
    Studiopolis1,
    Studiopolis2,
    FlyingBattery1,
    FlyingBattery2,
    PressGarden1,
    PressGarden2,
    StardustSpeedway1,
    StardustSpeedway2,
    Hydrocity1,
    Hydrocity2,
    MirageSaloon1,
    MirageSaloon2,
    OilOcean1,
    OilOcean2,
    LavaReef1,
    LavaReef2,
    MetallicMadness1,
    MetallicMadness2,
    TitanicMonarch1,
    TitanicMonarch2,
    EggReverie,
    EncoreAngelIsland,
    EncoreGreenHill1,
    EncoreGreenHill2,
    EncoreChemicalPlant1,
    EncoreChemicalPlant2,
    EncoreStudiopolis1,
    EncoreStudiopolis2,
    EncoreFlyingBattery1,
    EncoreFlyingBattery2,
    EncorePressGarden1,
    EncorePressGarden2,
    EncoreStardustSpeedway1,
    EncoreStardustSpeedway2,
    EncoreHydrocity1,
    EncoreHydrocity2,
    EncoreMirageSaloon1,
    EncoreMirageSaloon2,
    EncoreOilOcean1,
    EncoreOilOcean2,
    EncoreLavaReef1,
    EncoreLavaReef2,
    EncoreMetallicMadness1,
    EncoreMetallicMadness2,
    EncoreTitanicMonarch1,
    EncoreTitanicMonarch2,
}

#[derive(Clone, Copy, PartialEq)]
enum GameMode {
    Standard,
    Encore,
    Invalid,
}
