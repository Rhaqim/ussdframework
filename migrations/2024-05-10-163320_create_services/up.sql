CREATE TABLE services (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    function_name TEXT NOT NULL,
    function_url TEXT,
    data_key TEXT NOT NULL,
    service_code TEXT
);

CREATE TABLE screens (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    text TEXT NOT NULL,
    screen_type TEXT NOT NULL,
    default_next_screen TEXT NOT NULL,
    service_code TEXT,
    function TEXT,
    input_identifier TEXT,
    input_type TEXT
);

CREATE TABLE menu_items (
    id INTEGER PRIMARY KEY,
    screen_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    option TEXT NOT NULL,
    display_name TEXT NOT NULL,
    next_screen TEXT NOT NULL,
    FOREIGN KEY (screen_id) REFERENCES screens(id) ON DELETE CASCADE
);

CREATE TABLE router_options (
    id INTEGER PRIMARY KEY,
    screen_id INTEGER NOT NULL,
    router_option TEXT NOT NULL,
    next_screen TEXT NOT NULL,
    FOREIGN KEY (screen_id) REFERENCES screens(id) ON DELETE CASCADE
);
