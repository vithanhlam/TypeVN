#include "engine.h"
#include <glib-unix.h>
#include <signal.h>
#include <string.h>

enum {
    TYPEVN_ACT_COMMIT = 0,
    TYPEVN_ACT_PREEDIT = 1,
    TYPEVN_ACT_PASSTHROUGH = 2,
    TYPEVN_ACT_DELETE = 3,
    TYPEVN_ACT_RESET = 4,
    TYPEVN_ACT_COMMIT_THEN_PASS = 5,
    TYPEVN_ACT_NOTIFY = 6,
    TYPEVN_ACT_COMMIT_THEN_NOTIFY = 7
};

void *typevn_engine_new(void);
void typevn_engine_free(void *eng);
void typevn_engine_reset(void *eng);
int typevn_engine_get_method(void *eng);
int typevn_engine_get_english(void *eng);
unsigned int typevn_engine_get_preedit_delay_ms(void *eng);
void typevn_engine_set_method(void *eng, int vni);
void typevn_engine_set_english(void *eng, int en);
void typevn_engine_reload(void *eng);
int typevn_process_key(void *eng,
                       unsigned int keyval,
                       unsigned int keycode,
                       unsigned int modifiers,
                       char *out_text,
                       size_t out_cap,
                       unsigned int *delete_count);

typedef struct _IBusTypeVNEngine IBusTypeVNEngine;
typedef struct _IBusTypeVNEngineClass IBusTypeVNEngineClass;

struct _IBusTypeVNEngine {
    IBusEngine parent;
    void *core;
    char last_preedit[256];
    guint preedit_timeout_id;
};

struct _IBusTypeVNEngineClass {
    IBusEngineClass parent;
};

G_DEFINE_TYPE(IBusTypeVNEngine, ibus_typevn_engine, IBUS_TYPE_ENGINE)

static IBusTypeVNEngine *g_active_engine = NULL;
static void show_status(IBusEngine *engine, const char *msg);
static void register_typevn_props(IBusEngine *engine);
static void do_commit(IBusEngine *engine, IBusTypeVNEngine *self, const char *text);

static gboolean commit_idle_preedit_cb(gpointer data) {
    IBusTypeVNEngine *self = (IBusTypeVNEngine *)data;
    self->preedit_timeout_id = 0;
    if (self->last_preedit[0] != '\0') {
        IBusEngine *engine = IBUS_ENGINE(self);
        do_commit(engine, self, self->last_preedit);
        typevn_engine_reset(self->core);
    }
    return G_SOURCE_REMOVE;
}

static void hide_preedit(IBusEngine *engine, IBusTypeVNEngine *self) {
    if (self->preedit_timeout_id != 0) {
        g_source_remove(self->preedit_timeout_id);
        self->preedit_timeout_id = 0;
    }
    IBusText *empty = ibus_text_new_from_static_string("");
    ibus_engine_update_preedit_text(engine, empty, 0, FALSE);
    ibus_engine_hide_preedit_text(engine);
    self->last_preedit[0] = '\0';
}

static void show_preedit(IBusEngine *engine, IBusTypeVNEngine *self, const char *text) {
    if (strcmp(self->last_preedit, text) == 0) {
        return;
    }
    g_strlcpy(self->last_preedit, text, sizeof(self->last_preedit));
    IBusText *t = ibus_text_new_from_string(text);
    guint len = (guint)g_utf8_strlen(text, -1);
    ibus_text_append_attribute(t, IBUS_ATTR_TYPE_UNDERLINE,
                               IBUS_ATTR_UNDERLINE_NONE, 0, len);
    ibus_engine_update_preedit_text_with_mode(
        engine, t, len, TRUE, IBUS_ENGINE_PREEDIT_COMMIT);
    if (self->preedit_timeout_id != 0) {
        g_source_remove(self->preedit_timeout_id);
    }
    self->preedit_timeout_id = g_timeout_add(
        typevn_engine_get_preedit_delay_ms(self->core), commit_idle_preedit_cb, self);
}

static void do_commit(IBusEngine *engine, IBusTypeVNEngine *self, const char *text) {
    char tmp[256];
    g_strlcpy(tmp, text ? text : "", sizeof(tmp));
    if (tmp[0] != '\0') {
        IBusText *t = ibus_text_new_from_string(tmp);
        ibus_engine_commit_text(engine, t);
    }
    hide_preedit(engine, self);
}

static gboolean hide_aux_cb(gpointer data) {
    IBusEngine *engine = IBUS_ENGINE(data);
    ibus_engine_hide_auxiliary_text(engine);
    return G_SOURCE_REMOVE;
}

static void show_status(IBusEngine *engine, const char *msg) {
    IBusText *t = ibus_text_new_from_string(msg ? msg : "TypeVN");
    ibus_engine_update_auxiliary_text(engine, t, TRUE);
    g_timeout_add_seconds(1, hide_aux_cb, engine);
}

static void register_typevn_props(IBusEngine *engine) {
    IBusTypeVNEngine *self = (IBusTypeVNEngine *)engine;
    int vni = typevn_engine_get_method(self->core);
    int en = typevn_engine_get_english(self->core);
    IBusPropList *props = ibus_prop_list_new();
    IBusPropList *methods = ibus_prop_list_new();

    ibus_prop_list_append(
        methods,
        ibus_property_new("MethodTelex", PROP_TYPE_RADIO,
                          ibus_text_new_from_string("Telex"), NULL, NULL, TRUE, TRUE,
                          vni ? PROP_STATE_UNCHECKED : PROP_STATE_CHECKED, NULL));
    ibus_prop_list_append(
        methods,
        ibus_property_new("MethodVni", PROP_TYPE_RADIO,
                          ibus_text_new_from_string("VNI"), NULL, NULL, TRUE, TRUE,
                          vni ? PROP_STATE_CHECKED : PROP_STATE_UNCHECKED, NULL));
    ibus_prop_list_append(
        props,
        ibus_property_new("MethodMenu", PROP_TYPE_MENU,
                          ibus_text_new_from_string(vni ? "VNI" : "Telex"), NULL, NULL,
                          TRUE, TRUE, PROP_STATE_UNCHECKED, methods));
    ibus_prop_list_append(
        props,
        ibus_property_new("InputEnglish", PROP_TYPE_TOGGLE,
                          ibus_text_new_from_string("Anh (tắt dấu)"), NULL, NULL, TRUE,
                          TRUE, en ? PROP_STATE_CHECKED : PROP_STATE_UNCHECKED, NULL));
    ibus_engine_register_properties(engine, props);
}

static gboolean on_usr1(gpointer data) {
    (void)data;
    if (!g_active_engine || !g_active_engine->core) {
        return G_SOURCE_CONTINUE;
    }
    IBusEngine *engine = IBUS_ENGINE(g_active_engine);
    /* Config reload clears the Rust buffer; drop any stale preedit too. */
    typevn_engine_reload(g_active_engine->core);
    hide_preedit(engine, g_active_engine);
    int vni = typevn_engine_get_method(g_active_engine->core);
    int en = typevn_engine_get_english(g_active_engine->core);
    show_status(engine, en ? "TypeVN · Anh" : (vni ? "TypeVN · VNI" : "TypeVN · Việt"));
    register_typevn_props(engine);
    return G_SOURCE_CONTINUE;
}

static void ibus_typevn_property_activate(IBusEngine *engine, const gchar *prop_name,
                                          guint state) {
    IBusTypeVNEngine *self = (IBusTypeVNEngine *)engine;
    if (!prop_name) {
        return;
    }
    if (g_strcmp0(prop_name, "MethodTelex") == 0) {
        typevn_engine_set_method(self->core, 0);
        show_status(engine, "TypeVN · Telex");
    } else if (g_strcmp0(prop_name, "MethodVni") == 0) {
        typevn_engine_set_method(self->core, 1);
        show_status(engine, "TypeVN · VNI");
    } else if (g_strcmp0(prop_name, "InputEnglish") == 0) {
        typevn_engine_set_english(self->core, state == PROP_STATE_CHECKED);
        show_status(engine, state == PROP_STATE_CHECKED ? "TypeVN · Anh" : "TypeVN · Việt");
    }
    register_typevn_props(engine);
}

static gboolean
ibus_typevn_engine_process_key_event(IBusEngine *engine,
                                     guint keyval,
                                     guint keycode,
                                     guint modifiers) {
    IBusTypeVNEngine *self = (IBusTypeVNEngine *)engine;
    char text[256];
    unsigned int del = 0;
    int kind;

    if (modifiers & IBUS_RELEASE_MASK) {
        return FALSE;
    }
    if (modifiers & IBUS_IGNORED_MASK) {
        return FALSE;
    }

    text[0] = '\0';
    kind = typevn_process_key(self->core, keyval, keycode, modifiers, text, sizeof(text), &del);

    switch (kind) {
    case TYPEVN_ACT_PREEDIT:
        show_preedit(engine, self, text);
        return TRUE;
    case TYPEVN_ACT_COMMIT:
        do_commit(engine, self, text);
        return TRUE;
    case TYPEVN_ACT_NOTIFY:
        hide_preedit(engine, self);
        show_status(engine, text);
        register_typevn_props(engine);
        return TRUE;
    case TYPEVN_ACT_COMMIT_THEN_NOTIFY: {
        /* Packed as "commit\0notify" in the same buffer. */
        const char *notify = text + strlen(text) + 1;
        do_commit(engine, self, text);
        if (notify < text + sizeof(text) && notify[0] != '\0') {
            show_status(engine, notify);
        }
        register_typevn_props(engine);
        return TRUE;
    }
    case TYPEVN_ACT_RESET:
        typevn_engine_reset(self->core);
        hide_preedit(engine, self);
        return TRUE;
    case TYPEVN_ACT_DELETE:
        if (del > 0) {
            ibus_engine_delete_surrounding_text(engine, -(gint)del, del);
        }
        return TRUE;
    case TYPEVN_ACT_COMMIT_THEN_PASS:
        do_commit(engine, self, text);
        return FALSE;
    case TYPEVN_ACT_PASSTHROUGH:
    default:
        return FALSE;
    }
}

static void ibus_typevn_engine_focus_in(IBusEngine *engine) {
    IBusTypeVNEngine *self = (IBusTypeVNEngine *)engine;
    /* Always start a field with a clean composition — avoids stuck buffers
     * after apps that skip focus_out / hide preedit without reset. */
    typevn_engine_reset(self->core);
    hide_preedit(engine, self);
    IBUS_ENGINE_CLASS(ibus_typevn_engine_parent_class)->focus_in(engine);
}

static void ibus_typevn_engine_focus_out(IBusEngine *engine) {
    IBusTypeVNEngine *self = (IBusTypeVNEngine *)engine;
    if (self->last_preedit[0] != '\0') {
        do_commit(engine, self, self->last_preedit);
    }
    /* Always reset even when last_preedit was empty: core buffer and IBus
     * preedit can desync (reload, notify, app quirks). A stuck buffer with
     * Vietnamese chars makes later keys stay literal — "mất dấu". */
    typevn_engine_reset(self->core);
    hide_preedit(engine, self);
    IBUS_ENGINE_CLASS(ibus_typevn_engine_parent_class)->focus_out(engine);
}

static void ibus_typevn_engine_reset(IBusEngine *engine) {
    IBusTypeVNEngine *self = (IBusTypeVNEngine *)engine;
    typevn_engine_reset(self->core);
    hide_preedit(engine, self);
    IBUS_ENGINE_CLASS(ibus_typevn_engine_parent_class)->reset(engine);
}

static void ibus_typevn_engine_enable(IBusEngine *engine) {
    IBusTypeVNEngine *self = (IBusTypeVNEngine *)engine;
    /* Re-sync Việt/Anh + method from ~/.config/typevn/config. Fixes the case
     * where an in-memory English toggle left marks dead until restart. */
    typevn_engine_reload(self->core);
    hide_preedit(engine, self);
    register_typevn_props(engine);
    IBUS_ENGINE_CLASS(ibus_typevn_engine_parent_class)->enable(engine);
}

static void ibus_typevn_engine_disable(IBusEngine *engine) {
    IBusTypeVNEngine *self = (IBusTypeVNEngine *)engine;
    typevn_engine_reset(self->core);
    hide_preedit(engine, self);
    IBUS_ENGINE_CLASS(ibus_typevn_engine_parent_class)->disable(engine);
}

static void ibus_typevn_engine_init(IBusTypeVNEngine *self) {
    self->core = typevn_engine_new();
    self->last_preedit[0] = '\0';
    self->preedit_timeout_id = 0;
    g_active_engine = self;
    static gsize once = 0;
    if (g_once_init_enter(&once)) {
        g_unix_signal_add(SIGUSR1, on_usr1, NULL);
        g_once_init_leave(&once, 1);
    }
}

static void ibus_typevn_engine_dispose(GObject *object) {
    IBusTypeVNEngine *self = (IBusTypeVNEngine *)object;
    if (self->preedit_timeout_id != 0) {
        g_source_remove(self->preedit_timeout_id);
        self->preedit_timeout_id = 0;
    }
    if (self->core) {
        typevn_engine_free(self->core);
        self->core = NULL;
    }
    if (g_active_engine == self) {
        g_active_engine = NULL;
    }
    G_OBJECT_CLASS(ibus_typevn_engine_parent_class)->dispose(object);
}

static void ibus_typevn_engine_class_init(IBusTypeVNEngineClass *klass) {
    IBusEngineClass *engine_class = IBUS_ENGINE_CLASS(klass);
    GObjectClass *object_class = G_OBJECT_CLASS(klass);

    object_class->dispose = ibus_typevn_engine_dispose;
    engine_class->process_key_event = ibus_typevn_engine_process_key_event;
    engine_class->focus_in = ibus_typevn_engine_focus_in;
    engine_class->focus_out = ibus_typevn_engine_focus_out;
    engine_class->reset = ibus_typevn_engine_reset;
    engine_class->enable = ibus_typevn_engine_enable;
    engine_class->disable = ibus_typevn_engine_disable;
    engine_class->property_activate = ibus_typevn_property_activate;
}
