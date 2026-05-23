/// Should mirror RsPluginVTable in shim.cpp
#[repr(C)]
pub struct RsPluginVTable {
    pub on_load: Option<unsafe extern "C" fn(*mut (), *const Server)>,
    pub on_enable: Option<unsafe extern "C" fn(*mut (), *const Server)>,
    pub on_disable: Option<unsafe extern "C" fn(*mut (), *const Server)>,
    pub drop: Option<unsafe extern "C" fn(*mut ())>,
}

/// The metadata struct that mirrors RsPluginMeta in shim.cpp
#[repr(C)]
pub struct RsPluginMeta {
    pub name: *const std::ffi::c_char,
    pub version: *const std::ffi::c_char,
    pub description: *const std::ffi::c_char,
    pub author: *const std::ffi::c_char,
    pub website: *const std::ffi::c_char,
    pub prefix: *const std::ffi::c_char,
}

// Globals that shim.cpp reads after we fill them in
#[unsafe(no_mangle)]
pub static mut ENDSTONE_RS_VTABLE: RsPluginVTable = RsPluginVTable {
    on_load: None,
    on_enable: None,
    on_disable: None,
    drop: None,
};

#[unsafe(no_mangle)]
pub static mut ENDSTONE_RS_META: RsPluginMeta = RsPluginMeta {
    name: std::ptr::null(),
    version: std::ptr::null(),
    description: std::ptr::null(),
    author: std::ptr::null(),
    website: std::ptr::null(),
    prefix: std::ptr::null(),
};

#[unsafe(no_mangle)]
pub static mut ENDSTONE_RS_PLUGIN_PTR: *mut () = std::ptr::null_mut();

unsafe extern "C" {
    fn endstone_rs_init_plugin() -> *mut ();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_endstone_plugin() -> *mut () {
    unsafe { endstone_rs_init_plugin() }
}

#[repr(C)]
pub struct Server {
    _unused: [u8; 0],
}

unsafe impl cxx::ExternType for Server {
    type Id = cxx::type_id!("endstone::Server");
    type Kind = cxx::kind::Opaque;
}

#[repr(C)]
pub struct Logger {
    _unused: [u8; 0],
}

unsafe impl cxx::ExternType for Logger {
    type Id = cxx::type_id!("endstone::Logger");
    type Kind = cxx::kind::Opaque;
}

#[repr(C)]
pub struct Player {
    _unused: [u8; 0],
}

unsafe impl cxx::ExternType for Player {
    type Id = cxx::type_id!("endstone::Player");
    type Kind = cxx::kind::Opaque;
}

#[repr(C)]
pub struct CommandSender {
    _unused: [u8; 0],
}

unsafe impl cxx::ExternType for CommandSender {
    type Id = cxx::type_id!("endstone::CommandSender");
    type Kind = cxx::kind::Opaque;
}

#[repr(C)]
pub struct Level {
    _unused: [u8; 0],
}

unsafe impl cxx::ExternType for Level {
    type Id = cxx::type_id!("endstone::Level");
    type Kind = cxx::kind::Opaque;
}

#[repr(C)]
pub struct Dimension {
    _unused: [u8; 0],
}

unsafe impl cxx::ExternType for Dimension {
    type Id = cxx::type_id!("endstone::Dimension");
    type Kind = cxx::kind::Opaque;
}

#[repr(C)]
pub struct Scoreboard {
    _unused: [u8; 0],
}

unsafe impl cxx::ExternType for Scoreboard {
    type Id = cxx::type_id!("endstone::Scoreboard");
    type Kind = cxx::kind::Opaque;
}

#[repr(C)]
pub struct Objective {
    _unused: [u8; 0],
}

unsafe impl cxx::ExternType for Objective {
    type Id = cxx::type_id!("endstone::Objective");
    type Kind = cxx::kind::Opaque;
}

#[repr(C)]
pub struct Inventory {
    _unused: [u8; 0],
}

unsafe impl cxx::ExternType for Inventory {
    type Id = cxx::type_id!("endstone::Inventory");
    type Kind = cxx::kind::Opaque;
}

#[repr(C)]
pub struct PlayerInventory {
    _unused: [u8; 0],
}

unsafe impl cxx::ExternType for PlayerInventory {
    type Id = cxx::type_id!("endstone::PlayerInventory");
    type Kind = cxx::kind::Opaque;
}

#[cxx::bridge(namespace = "endstone")]
mod logger_ffi {
    struct UUID {
        pub data: [u8; 16],
    }

    struct Vector {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    enum DisplaySlot {
        BelowName,
        PlayerList,
        SideBar,
    }

    enum RenderType {
        Integer,
        Hearts,
    }

    enum CriteriaType {
        Dummy,
    }

    unsafe extern "C++" {
        include!("endstone/logger.h");
        include!("endstone/server.h");
        include!("endstone/player.h");
        include!("endstone/command/command_sender.h");
        include!("endstone/level/level.h");
        include!("endstone/level/dimension.h");
        include!("endstone/scoreboard/scoreboard.h");
        include!("endstone/scoreboard/criteria.h");
        include!("endstone/inventory/inventory.h");
        include!("endstone/inventory/player_inventory.h");
        include!("endstone/util/uuid.h");
        include!("endstone/util/vector.h");

        type Logger = crate::bridge::Logger;
        fn info(self: &Logger, message: &CxxString);
        fn warning(self: &Logger, message: &CxxString);
        fn error(self: &Logger, message: &CxxString);
        fn trace(self: &Logger, message: &CxxString);
        fn debug(self: &Logger, message: &CxxString);
        fn critical(self: &Logger, message: &CxxString);

        type Server = crate::bridge::Server;
        #[rust_name = "get_name"]
        fn getName(self: &Server) -> String;
        #[rust_name = "get_version"]
        fn getVersion(self: &Server) -> String;
        #[rust_name = "get_minecraft_version"]
        fn getMinecraftVersion(self: &Server) -> String;
        #[rust_name = "get_protocol_version"]
        fn getProtocolVersion(self: &Server) -> i32;
        #[rust_name = "get_logger"]
        fn getLogger(self: &Server) -> &Logger;
        #[rust_name = "get_port"]
        fn getPort(self: &Server) -> i32;
        #[rust_name = "get_port_v6"]
        fn getPortV6(self: &Server) -> i32;
        #[rust_name = "get_online_mode"]
        fn getOnlineMode(self: &Server) -> bool;
        #[rust_name = "get_max_players"]
        fn getMaxPlayers(self: &Server) -> i32;
        #[rust_name = "set_max_players"]
        fn setMaxPlayers(self: &Server, max_players: i32);
        fn shutdown(self: &Server);
        fn reload(self: &Server);
        #[rust_name = "reload_data"]
        fn reloadData(self: &Server);
        #[rust_name = "is_primary_thread"]
        fn isPrimaryThread(self: &Server) -> bool;
        #[rust_name = "get_current_milliseconds_per_tick"]
        fn getCurrentMillisecondsPerTick(self: &Server) -> f32;
        #[rust_name = "get_average_milliseconds_per_tick"]
        fn getAverageMillisecondsPerTick(self: &Server) -> f32;
        #[rust_name = "get_current_ticks_per_second"]
        fn getCurrentTicksPerSecond(self: &Server) -> f32;
        #[rust_name = "get_average_ticks_per_second"]
        fn getAverageTicksPerSecond(self: &Server) -> f32;
        #[rust_name = "get_current_tick_usage"]
        fn getCurrentTickUsage(self: &Server) -> f32;
        #[rust_name = "get_average_tick_usage"]
        fn getAverageTickUsage(self: &Server) -> f32;
        #[rust_name = "broadcast_message"]
        fn broadcastMessage(self: &Server, message: &CxxString);
        #[rust_name = "get_scoreboard"]
        fn getScoreboard(self: &Server) -> *mut Scoreboard;
        #[rust_name = "get_level"]
        fn getLevel(self: &Server) -> &Level;
        #[rust_name = "get_player_by_name"]
        fn getPlayer(self: &Server, name: String) -> &Player;
        #[rust_name = "get_player_by_uuid"]
        fn getPlayer(self: &Server, id: UUID) -> &Player;
        #[rust_name = "get_online_players_count"]
        fn get_online_players_count(server: &Server) -> usize;
        #[rust_name = "get_online_player_at"]
        fn get_online_player_at(server: &Server, index: usize) -> &Player;

        type Player = crate::bridge::Player;
        #[rust_name = "get_name"]
        fn getName(self: &Player) -> String;
        #[rust_name = "is_op"]
        fn isOp(self: &Player) -> bool;
        #[rust_name = "set_op"]
        fn setOp(self: &Player, value: bool);
        #[rust_name = "get_xuid"]
        fn getXuid(self: &Player) -> String;
        fn transfer(self: &Player, host: String, port: i32);
        fn kick(self: &Player, message: String);
        #[rust_name = "perform_command"]
        fn performCommand(self: &Player, command: String) -> bool;
        #[rust_name = "is_sneaking"]
        fn isSneaking(self: &Player) -> bool;
        #[rust_name = "set_sneaking"]
        fn setSneaking(self: &Player, sneak: bool);
        #[rust_name = "is_sprinting"]
        fn isSprinting(self: &Player) -> bool;
        #[rust_name = "set_sprinting"]
        fn setSprinting(self: &Player, sprinting: bool);
        #[rust_name = "give_exp"]
        fn giveExp(self: &Player, amount: i32);
        #[rust_name = "give_exp_levels"]
        fn giveExpLevels(self: &Player, amount: i32);
        #[rust_name = "get_exp_progress"]
        fn getExpProgress(self: &Player) -> f32;
        #[rust_name = "set_exp_progress"]
        fn setExpProgress(self: &Player, progress: f32);
        #[rust_name = "get_exp_level"]
        fn getExpLevel(self: &Player) -> i32;
        #[rust_name = "set_exp_level"]
        fn setExpLevel(self: &Player, level: i32);
        #[rust_name = "get_total_exp"]
        fn getTotalExp(self: &Player) -> i32;
        #[rust_name = "get_allow_flight"]
        fn getAllowFlight(self: &Player) -> bool;
        #[rust_name = "set_allow_flight"]
        fn setAllowFlight(self: &Player, flight: bool);
        #[rust_name = "is_flying"]
        fn isFlying(self: &Player) -> bool;
        #[rust_name = "set_flying"]
        fn setFlying(self: &Player, value: bool);
        #[rust_name = "get_fly_speed"]
        fn getFlySpeed(self: &Player) -> f32;
        #[rust_name = "set_fly_speed"]
        fn setFlySpeed(self: &Player, value: f32);
        #[rust_name = "get_walk_speed"]
        fn getWalkSpeed(self: &Player) -> f32;
        #[rust_name = "set_walk_speed"]
        fn setWalkSpeed(self: &Player, value: f32);
        #[rust_name = "get_scoreboard"]
        fn getScoreboard(self: &Player) -> &Scoreboard;
        #[rust_name = "set_scoreboard"]
        fn setScoreboard(self: &Player, scoreboard: &Scoreboard);
        #[rust_name = "send_popup"]
        fn sendPopup(self: &Player, message: String);
        #[rust_name = "send_tip"]
        fn sendTip(self: &Player, message: String);
        #[rust_name = "send_toast"]
        fn sendToast(self: &Player, title: String, content: String);
        #[rust_name = "send_title"]
        fn sendTitle(self: &Player, title: String, subtitle: String);
        #[rust_name = "reset_title"]
        fn resetTitle(self: &Player);
        #[rust_name = "get_locale"]
        fn getLocale(self: &Player) -> String;
        #[rust_name = "update_commands"]
        fn updateCommands(self: &Player);
        #[rust_name = "get_device_os"]
        fn getDeviceOS(self: &Player) -> String;
        #[rust_name = "get_device_id"]
        fn getDeviceId(self: &Player) -> String;
        #[rust_name = "get_game_version"]
        fn getGameVersion(self: &Player) -> String;
        #[rust_name = "get_inventory"]
        fn getInventory(self: &Player) -> &PlayerInventory;
        #[rust_name = "get_ender_chest"]
        fn getEnderChest(self: &Player) -> &Inventory;
        #[rust_name = "close_form"]
        fn closeForm(self: &Player);

        type Inventory = crate::bridge::Inventory;
        #[rust_name = "get_size"]
        fn getSize(self: &Inventory) -> i32;

        type PlayerInventory = crate::bridge::PlayerInventory;
        #[rust_name = "get_held_item_slot"]
        fn getHeldItemSlot(self: &PlayerInventory) -> i32;
        #[rust_name = "set_held_item_slot"]
        fn setHeldItemSlot(self: &PlayerInventory, slot: i32);

        type CommandSender = crate::bridge::CommandSender;
        #[rust_name = "send_message"]
        fn sendMessage(self: &CommandSender, message: &CxxString);
        #[rust_name = "send_error_message"]
        fn sendErrorMessage(self: &CommandSender, message: &CxxString);
        #[rust_name = "get_server"]
        fn getServer(self: &CommandSender) -> &Server;
        #[rust_name = "get_name"]
        fn getName(self: &CommandSender) -> String;

        type Level = crate::bridge::Level;
        #[rust_name = "get_name"]
        fn getName(self: &Level) -> String;
        #[rust_name = "get_time"]
        fn getTime(self: &Level) -> i32;
        #[rust_name = "set_time"]
        fn setTime(self: &Level, time: i32);
        #[rust_name = "get_dimensions_count"]
        fn get_dimensions_count(level: &Level) -> usize;
        #[rust_name = "get_dimension_at"]
        fn get_dimension_at(level: &Level, index: usize) -> &Dimension;

        type Scoreboard = crate::bridge::Scoreboard;
        #[rust_name = "clear_slot"]
        fn clearSlot(self: &Scoreboard, slot: DisplaySlot);

        type Objective = crate::bridge::Objective;
        #[rust_name = "get_name"]
        fn getName(self: &Objective) -> String;
        #[rust_name = "get_display_name"]
        fn getDisplayName(self: &Objective) -> String;
        #[rust_name = "set_display_name"]
        fn setDisplayName(self: &Objective, name: String);
        #[rust_name = "get_render_type"]
        fn getRenderType(self: &Objective) -> RenderType;
        #[rust_name = "set_render_type"]
        fn setRenderType(self: &Objective, render_type: RenderType);
        #[rust_name = "get_display_slot"]
        fn getDisplaySlot(self: &Objective) -> DisplaySlot;
        #[rust_name = "set_display_slot"]
        fn setDisplaySlot(self: &Objective, slot: DisplaySlot);
        fn unregister(self: &Objective);

        type Dimension = crate::bridge::Dimension;
        #[rust_name = "get_name"]
        fn getName(self: &Dimension) -> String;
    }
}