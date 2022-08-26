use crate::interactable::{Interactable, InteractableType};
use crate::player::fsm::{PlayerState, PlayerStateMachine};
use crate::player::PlayerLookingAt;
use crate::prelude::*;
use crate::viewmodel::ViewModelHold;
use bevy_asset_loader::prelude::AssetCollection;
use iyes_loopless::prelude::AppLooplessStateExt;

#[derive(AssetCollection)]
pub struct UiAssets {
    #[asset(path = "fonts/mono.ttf")]
    pub font: Handle<Font>,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::MainMenu, spawn_gui);
    }
}

#[derive(Component)]
pub struct InteractText;

#[derive(Component)]
pub struct TimerText;

#[derive(Component)]
pub struct Crosshair;

pub fn spawn_gui(ui_assets: Res<UiAssets>, mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|b| {
            // crosshair
            b.spawn()
                .insert_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|crsh| {
                    crsh.spawn()
                        .insert_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::ColumnReverse,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|b| {
                            b.spawn().insert_bundle(
                                TextBundle::from_section(
                                    "you found the hidden owo! ZE PENG STRIKES AGAIN! FACE THE GIT MERGE THE CODE",
                                    TextStyle {
                                        font: ui_assets.font.clone(),
                                        font_size: 30.0,
                                        color: Color::NONE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..Default::default()
                                }),
                            );
                            b.spawn().insert_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(7.0), Val::Px(7.0)),
                                    ..Default::default()
                                },
                                color: Color::WHITE.into(),
                                ..Default::default()
                            }).insert(Crosshair);
                            b.spawn().insert_bundle(
                                TextBundle::from_section(
                                    "[MOUSE1] to fucking DIE",
                                    TextStyle {
                                        font: ui_assets.font.clone(),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..Default::default()
                                }),
                            ).insert(InteractText);
                        });
                });
            // timer top right
            b.spawn()
                .insert_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Auto),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        align_self: AlignSelf::FlexEnd,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    color: Color::rgba(0.0,0.0,0.0,0.5).into(),
                    ..Default::default()
                })
                .with_children(|b| {
                    b.spawn().insert_bundle(TextBundle::from_section(
                        "YOU WILL BE YEETED IN: 10:10",
                        TextStyle {
                            font: ui_assets.font.clone(),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    ).with_style(Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    }))
                    .insert(TimerText);
                });

        });
}

enum UiInteractable {
    Hammer,
    LineOfCode,
    LineOfCodeGlobule,
    Terminal,
    None,
}

impl From<Interactable> for UiInteractable {
    fn from(i: Interactable) -> Self {
        match i.itype {
            InteractableType::Hammer => UiInteractable::Hammer,
            InteractableType::LineOfCode => UiInteractable::LineOfCode,
            InteractableType::LineOfCodeGlobule => UiInteractable::LineOfCodeGlobule,
            InteractableType::Terminal => UiInteractable::Terminal,
        }
    }
}

pub fn update_interact_text(
    player_state: Res<PlayerStateMachine>,
    looking_at: Res<PlayerLookingAt>,
    viewmodel_holding: Query<&ViewModelHold>,
    interactable: Query<&Interactable>,
    mut text: Query<&mut Text, With<InteractText>>,
) {
    let mut itext = text.single_mut();
    if player_state.state() == PlayerState::Interacting {
        itext.sections[0].value = "".to_string();
    }

    let holding = viewmodel_holding.single();

    match looking_at.entity {
        Some(e) => {
            let interactable = match interactable.get(e) {
                Ok(v) => (*v).into(),
                Err(_) => UiInteractable::None,
            };

            match (holding, interactable) {
                (_, UiInteractable::Terminal) => {
                    itext.sections[0].value = "[MOUSE1] Interact".to_string();
                }
                (ViewModelHold::Empty, UiInteractable::None) => {
                    itext.sections[0].value = "".to_string();
                }
                (ViewModelHold::Empty, _) => {
                    itext.sections[0].value = "[MOUSE1] Pickup".to_string();
                }
                (ViewModelHold::Hammer, UiInteractable::None) => {
                    itext.sections[0].value = "[MOUSE1] Swing".to_string();
                }
                (
                    ViewModelHold::Hammer,
                    UiInteractable::LineOfCodeGlobule | UiInteractable::LineOfCode,
                ) => {
                    itext.sections[0].value = "[MOUSE1] Swing\n[MOUSE2] Swap".to_string();
                }
                (
                    ViewModelHold::LoC | ViewModelHold::LoCBundle,
                    UiInteractable::LineOfCode | UiInteractable::LineOfCodeGlobule,
                ) => {
                    itext.sections[0].value = "[MOUSE1] Attach\n[MOUSE2] Swap".to_string();
                }
                (_, _) => {}
            }
        }
        None => match holding {
            ViewModelHold::Empty => {
                itext.sections[0].value = "".to_string();
            }
            _ => {
                itext.sections[0].value = "[MOUSE1] Swing\n[MOUSE2] Throw".to_string();
            }
        },
    };
}