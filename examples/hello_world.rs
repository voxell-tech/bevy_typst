use bevy::prelude::*;
use bevy_typst::prelude::*;
use bevy_vello::{VelloAssetBundle, VelloPlugin};
use typst::foundations::Label;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        // Custom plugins
        .add_plugins((TypstPlugin::default(), VelloPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (check_document, check_module, check_svg))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        asset_server.load::<TypstDocAsset>("hello_world.typ"),
        asset_server.load::<TypstModAsset>("hello_world.typ"),
        asset_server.load::<SvgAsset>("hello_world.typ"),
        VelloAssetBundle {
            vector: asset_server.load("hello_world.typ"),
            ..default()
        },
    ));
}

fn check_document(
    mut commands: Commands,
    q_typst_asset: Query<(Entity, &Handle<TypstDocAsset>)>,
    typst_doc_assets: Res<Assets<TypstDocAsset>>,
) {
    let Ok((entity, handle)) = q_typst_asset.get_single() else {
        return;
    };

    if typst_doc_assets.get(handle).is_some() {
        info!("Has document.");
        commands.entity(entity).remove::<Handle<TypstDocAsset>>();
    }
}

fn check_module(
    mut commands: Commands,
    q_typst_asset: Query<(Entity, &Handle<TypstModAsset>)>,
    typst_mod_assets: Res<Assets<TypstModAsset>>,
) {
    let Ok((entity, handle)) = q_typst_asset.get_single() else {
        return;
    };

    if let Some(module) = typst_mod_assets.get(handle).map(|asset| asset.module()) {
        info!("Has module.");
        let typst_title =
            module
                .clone()
                .content()
                .query_first(typst::foundations::Selector::Label(Label::new(
                    "typst_title",
                )));
        println!("typst_title: {typst_title:?}");
        commands.entity(entity).remove::<Handle<TypstModAsset>>();
    }
}

fn check_svg(
    mut commands: Commands,
    q_svg_asset: Query<(Entity, &Handle<SvgAsset>)>,
    svg_assets: Res<Assets<SvgAsset>>,
) {
    let Ok((entity, handle)) = q_svg_asset.get_single() else {
        return;
    };

    if svg_assets.get(handle).is_some() {
        info!("Has tree.");
        commands.entity(entity).remove::<Handle<SvgAsset>>();
    }
}
