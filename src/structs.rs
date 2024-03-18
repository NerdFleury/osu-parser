pub(crate) mod replay_structs {
    pub struct Replay {
        pub(crate) mode: u8,
        pub(crate) version: u32,
        pub(crate) osu_md5: String,
        pub(crate) player_name: String,
        pub(crate) replay_md5: String,
        pub(crate) count_300: u16,
        pub(crate) count_100: u16,
        pub(crate) count_50: u16,
        pub(crate) count_geki: u16,
        pub(crate) count_katu: u16,
        pub(crate) count_miss: u16,
        pub(crate) score: u32,
        pub(crate) greatest_combo: u16,
        pub(crate) perfect_combo: bool,
        pub(crate) mods: u32,
        pub(crate) life_bar_graph: String,
        pub(crate) timestamp: u64,
        pub(crate) compressed_replay_length: u32,
        pub(crate) compressed_replay_data: Vec<u8>,
        pub(crate) online_score_id: u64,
    }

    pub struct ReplayData {
        pub(crate) time_since_last_action: i64,
        pub(crate) x_position: f32,
        pub(crate) y_position: f32,
        pub(crate) keys_and_buttons: u32,
    }
}
