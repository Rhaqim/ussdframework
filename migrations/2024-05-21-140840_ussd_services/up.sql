-- Create services table
CREATE TABLE IF NOT EXISTS services (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    function_name TEXT NOT NULL,
    function_url TEXT,
    data_key TEXT NOT NULL,
    service_code TEXT
);

-- Create screens table
CREATE TABLE IF NOT EXISTS screens (
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

-- Create menu_items table
CREATE TABLE IF NOT EXISTS menu_items (
    id INTEGER PRIMARY KEY,
    screen_name TEXT NOT NULL,
    name TEXT NOT NULL,
    option TEXT NOT NULL,
    display_name TEXT NOT NULL,
    next_screen TEXT NOT NULL
);

-- Create router_options table
CREATE TABLE IF NOT EXISTS router_options (
    id INTEGER PRIMARY KEY,
    screen_name TEXT NOT NULL,
    router_option TEXT NOT NULL,
    next_screen TEXT NOT NULL
);
