use bevy::prelude::*;
use crate::AppState;

pub struct FinishLinePlugin;
impl Plugin for FinishLinePlugin {
    fn build(&self, app: &mut App) {
 //       app.add_systems(Update, detect_finish_line.run_if(in_state(AppState::InGame)));
    }
}


/*
   should make 4 invisible walls for start, quarter, halfway, 3/4ths, and then have a detector  
   each kart should track where the next wall they need to hit, like an enum
   when a hit is detected, change the enum on the kart for their next target
   we might also be able to detect when a kart is going backward by seeing if they hit a wall they shouldn't
   when the kart crosses the finish line after crossing the other walls, increment their lap count

*/



#[derive(Component)]
pub struct FinishLine;

fn detect_finish_line(
    finish_lines: Query<&Transform, With<FinishLine>>,
//    mut finish_line_manager: ResMut<FinishLineManager>,
) {
}
