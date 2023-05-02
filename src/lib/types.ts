export interface FieldProps {
    name: string;
    value: string;
    editable: boolean;
    field_type: string;
    format_error_message: string;
}

export interface Game {
    id: string;
    directory: string;
    fields: FieldProps[];
}

export interface Games {
    emulator: string;
    directory: string;
    games: Game[];
}

export interface Emulator {
    complete_percent: number;
    name: string;
    directory: string;
    game_count: number;
    gamefile_elements: number;
    emulator: string;
    rom_extensions: string[];
}
export interface Emulators {
    emulators: Emulator[];
}
