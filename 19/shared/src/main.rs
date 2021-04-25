type PlayerId = u32;

const GAME_SIZE: usize = 8;

type WaitingList = Vec<PlayerId>;

use std::sync::Arc;
use std::sync::Mutex;

struct FermEmpireApp {
    waiting_list: Mutex<WaitingList>,
}

impl FermEmpireApp {
    fn join_waiting_list(&self, player: PlayerId) {
        let mut guard = self.waiting_list.lock().unwrap();

        guard.push(player);
        if guard.len() == GAME_SIZE {
            let players = guard.split_off(0);
            self.start_game(players);
        }
    }

    fn start_game(&self, _players: Vec<PlayerId>) {
        panic!("Smile and wave")
    }
}

fn main() {
    let app = Arc::new(FermEmpireApp {
        waiting_list: Mutex::new(vec![]),
    });

    app.join_waiting_list(2);
}
