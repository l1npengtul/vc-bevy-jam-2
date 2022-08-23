use crate::office::OfficeAssets;
use crate::prelude::*;

pub mod conv_cp437;
mod text_sprite;
pub use text_sprite::*;
mod screen;
pub use screen::TerminalScreenTarget;

pub const TERM_DIM: (f32, f32) = (1280.0, 960.0);
pub const TERM_W: f32 = TERM_DIM.0;
pub const TERM_H: f32 = TERM_DIM.1;

pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TextSpritePlugin)
            .add_enter_system(GameState::MainMenu, TerminalInput::spawn)
            .add_enter_system(GameState::MainMenu, TerminalScreenTarget::set_up_2d)
            .add_system(TerminalInput::take_input.run_in_state(GameState::MainMenu));
    }
}

#[derive(Component)]
pub struct TerminalInput;

impl TerminalInput {
    fn spawn(
        mut commands: Commands,
        font: Res<FontAtlas>,
        target: Res<TerminalScreenTarget>,
        office: Res<OfficeAssets>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let prompt = TextSprite::new("input: ".to_string(), font.atlas.clone(), 1.0);
        prompt.spawn(
            &mut commands,
            |_| {},
            |mut parent| {
                parent.insert(TerminalInput);
                parent.insert(Transform::from_xyz(
                    (ATLAS_CHAR_W - TERM_W) / 2.0,
                    (TERM_H - ATLAS_CHAR_H) / 2.,
                    0.,
                ));
            },
        );
        let target_material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(target.image.clone()),
            reflectance: 0.02,
            unlit: false,
            ..Default::default()
        });

        let builder = office.assets.get(&"render_target".to_owned()).unwrap();

        // The cube that will be rendered to the texture.
        commands.spawn_bundle(MaterialMeshBundle {
            mesh: builder.collider_mesh.clone().unwrap(),
            material: target_material_handle,
            transform: builder.trans,
            ..Default::default()
        });
    }

    fn take_input(
        mut commands: Commands,
        mut q_input: Query<(Entity, &mut TextSprite), With<TerminalInput>>,
        mut keystrokes: EventReader<ReceivedCharacter>,
        keys: Res<Input<KeyCode>>,
    ) {
        let (entity, mut text_sprite) = q_input.single_mut();
        let input = keystrokes
            .iter()
            .map(|ev| ev.char)
            .filter(|ch| conv_cp437::index_of(*ch).is_some())
            .collect::<String>();
        text_sprite.add_str(&input, &mut commands, entity, |_| {});
        if keys.just_pressed(KeyCode::Back) {
            text_sprite.pop(&mut commands);
        }
        if keys.just_pressed(KeyCode::Return) {
            text_sprite.push_newline()
        }
    }
}
