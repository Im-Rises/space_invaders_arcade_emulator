/*mod binary_lib;
pub mod si_arcade;

/*
To do list:
- Add memory mirror
 */

fn main() {
    let mut space_invaders_arcade = si_arcade::SpaceInvadersArcade::new("romsPaths");
    space_invaders_arcade.start();
}
*/

use fermium::{
    prelude::{SDL_CreateRenderer, SDL_Delay, SDL_RenderClear, SDL_RenderPresent, SDL_SetRenderDrawColor},
    video::*,
    *,
};

// Tested on Windows with msvc compiler toolchain
pub fn main() {
    unsafe {
        assert_eq!(SDL_Init(SDL_INIT_EVERYTHING), 0);

        let window = SDL_CreateWindow(
            b"demo\0".as_ptr().cast(),
            SDL_WINDOWPOS_CENTERED,
            SDL_WINDOWPOS_CENTERED,
            800,
            600,
            (SDL_WINDOW_OPENGL | SDL_WINDOW_ALLOW_HIGHDPI).0,
        );
        // Panic if window is not null
        assert!(!window.is_null());

        let renderer = SDL_CreateRenderer(window, -1, 1);
        // Panic if renderer is not null
        assert!(!renderer.is_null());
        SDL_SetRenderDrawColor(renderer, 0, 0, 255, 255);

        loop {
            SDL_RenderClear(renderer);

            SDL_RenderPresent(renderer);

            SDL_Delay(10);
        }

        SDL_DestroyWindow(window);
        SDL_Quit();
    }
}
