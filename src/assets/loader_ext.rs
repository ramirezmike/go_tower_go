pub mod command_ext {
    use crate::{
        assets::loader::QueueState,
        AppState, ingame, menu,
    };
    use bevy::ecs::system::{Command, SystemState};
    use bevy::prelude::*;

    pub trait CommandLoading {
        fn load_state(&mut self, state: AppState);
    }

    impl<'w, 's> CommandLoading for Commands<'w, 's> {
        fn load_state(&mut self, state: AppState) {
            self.add(StateSetter(state));
        }
    }

    pub struct StateSetter(AppState);
    impl Command for StateSetter {
        fn apply(self, world: &mut World) {
            let mut system_state: SystemState<(ResMut<QueueState>, ResMut<NextState<AppState>>)> =
                SystemState::new(world);
            let (mut queued_state, mut next_state) = system_state.get_mut(world);

            queued_state.state = self.0;
            next_state.set(AppState::Loading);

            #[cfg(feature = "debug")]
            {
                // Add all loaders here
                menu::splash::SplashLoader.apply(world);
                menu::title_screen::loader::TitleScreenLoader.apply(world);
                menu::controls::loader::ControlsLoader.apply(world);
                menu::instructions::loader::InstructionsLoader.apply(world);
                ingame::IngameLoader.apply(world);
                // return;
            }

            match self.0 {
                AppState::InGame => ingame::IngameLoader.apply(world),
                AppState::Splash => menu::splash::SplashLoader.apply(world),
                AppState::TitleScreen => menu::title_screen::loader::TitleScreenLoader.apply(world),
                AppState::Controls => menu::controls::loader::ControlsLoader.apply(world),
                AppState::Instructions => menu::instructions::loader::InstructionsLoader.apply(world),
                _ => (),
            }
        }
    }
}
