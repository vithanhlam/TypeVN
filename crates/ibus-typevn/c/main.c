#include "engine.h"
#include <stdlib.h>
#include <string.h>

static IBusBus *bus = NULL;
static IBusFactory *factory = NULL;

int typevn_ibus_main(int argc, char **argv) {
    ibus_init();

    bus = ibus_bus_new();
    if (!ibus_bus_is_connected(bus)) {
        g_printerr("typevn: cannot connect to IBus. Is ibus-daemon running?\n");
        return 1;
    }

    factory = ibus_factory_new(ibus_bus_get_connection(bus));
    g_object_ref_sink(factory);
    ibus_factory_add_engine(factory, "typevn", IBUS_TYPE_TYPEVN_ENGINE);

    gboolean exec_by_ibus = FALSE;
    for (int i = 1; i < argc; i++) {
        if (g_strcmp0(argv[i], "--ibus") == 0) {
            exec_by_ibus = TRUE;
        }
    }

    if (exec_by_ibus) {
        ibus_bus_request_name(bus, "org.freedesktop.IBus.TypeVN", 0);
    } else {
        IBusComponent *component = ibus_component_new(
            "org.freedesktop.IBus.TypeVN",
            "TypeVN Vietnamese Input",
            "0.1.0",
            "MIT",
            "vithanhlam",
            "https://localhost",
            argv[0],
            "typevn");
        IBusEngineDesc *desc = ibus_engine_desc_new(
            "typevn",
            "TypeVN",
            "Bộ gõ tiếng Việt TypeVN (Telex)",
            "vi",
            "MIT",
            "vithanhlam",
            "ibus-keyboard",
            "us");
        ibus_component_add_engine(component, desc);
        ibus_bus_register_component(bus, component);
        g_object_unref(component);
    }

    ibus_main();
    return 0;
}
