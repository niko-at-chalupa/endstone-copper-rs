#define FMT_HEADER_ONLY
#include <endstone/player.h>
#include <endstone/plugin/plugin.h>
#include <endstone/plugin/plugin_description.h>
#include <cstring>
#include <string>

// ── Types the Rust side fills in ─────────────────────────────────────────────

struct RsPluginVTable {
    void (*on_load)(void *rust_plugin);
    void (*on_enable)(void *rust_plugin);
    void (*on_disable)(void *rust_plugin);
    void (*drop)(void *rust_plugin);  // destructor
};

struct RsPluginMeta {
    const char *name;
    const char *version;
    const char *description;  // nullable
    const char *author;       // nullable
    const char *website;      // nullable
    const char *prefix;       // nullable
};

// Set by Rust before init_endstone_plugin() returns (see ffi.rs)
extern "C" {
    // Rust fills these in via endstone_register_plugin()
    extern RsPluginVTable ENDSTONE_RS_VTABLE;
    extern RsPluginMeta ENDSTONE_RS_META;
    extern void *ENDSTONE_RS_PLUGIN_PTR; // Box<T> raw ptr
}

// ── The C++ plugin wrapper ────────────────────────────────────────────────────

class RustPluginBridge final : public endstone::Plugin {
public:
    explicit RustPluginBridge(
        endstone::PluginDescription desc,
        RsPluginVTable vtable,
        void *rust_ptr
    ) : desc_(std::move(desc)), vtable_(vtable), rust_ptr_(rust_ptr) {}

    ~RustPluginBridge() override {
        if (vtable_.drop) vtable_.drop(rust_ptr_);
    }

    [[nodiscard]] const endstone::PluginDescription &getDescription() const override {
        return desc_;
    }

    void onLoad() override {
        if (vtable_.on_load) vtable_.on_load(rust_ptr_);
    }

    void onEnable() override {
        if (vtable_.on_enable) vtable_.on_enable(rust_ptr_);
    }

    void onDisable() override {
        if (vtable_.on_disable) vtable_.on_disable(rust_ptr_);
    }

private:
    endstone::PluginDescription desc_;
    RsPluginVTable vtable_;
    void *rust_ptr_;
};

// ── Entry point Endstone calls ────────────────────────────────────────────────

// Declared by Rust, called once to let Rust set up the globals above
extern "C" void endstone_rs_init();

extern "C" [[maybe_unused]]
endstone::Plugin *endstone_rs_init_plugin() {
    // Let Rust fill in the vtable/meta globals
    endstone_rs_init();

    // Build a PluginDescription from the meta Rust provided
    auto &m = ENDSTONE_RS_META;

    std::vector<std::string> authors;
    if (m.author) {
        authors.emplace_back(m.author);
    }

    endstone::PluginDescription desc(
        m.name ? m.name : "unknown",
        m.version ? m.version : "0.0.0",
        m.description ? m.description : "",
        endstone::PluginLoadOrder::PostWorld,
        authors,
        {}, // contributors
        m.website ? m.website : "",
        m.prefix ? m.prefix : ""
    );

    return new RustPluginBridge(
        std::move(desc),
        ENDSTONE_RS_VTABLE,
        ENDSTONE_RS_PLUGIN_PTR
    );
}

// ── Helpers called by Rust ────────────────────────────────────────────────────

extern "C" void endstone_player_send_message(endstone::Player *player, const char *message) {
    if (player && message) {
        player->sendMessage(message);
    }
}