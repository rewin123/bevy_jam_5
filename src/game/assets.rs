use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HandleMap<ImageKey>>();
    app.init_resource::<HandleMap<ImageKey>>();

    app.register_type::<HandleMap<SfxKey>>();
    app.init_resource::<HandleMap<SfxKey>>();

    app.register_type::<HandleMap<SoundtrackKey>>();
    app.init_resource::<HandleMap<SoundtrackKey>>();

    app.register_type::<HandleMap<SceneKey>>();
    app.init_resource::<HandleMap<SceneKey>>();

    app.register_type::<HandleMap<FontKey>>();
    app.init_resource::<HandleMap<FontKey>>();
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum FontKey {
    Pixel,
}

impl AssetKey for FontKey {
    type Asset = Font;
}

impl FromWorld for HandleMap<FontKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(
            FontKey::Pixel,
            asset_server.load("fonts/upheaval-tt-brk/upheavtt.ttf"),
        )]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum ImageKey {
    Ducky,
}

impl AssetKey for ImageKey {
    type Asset = Image;
}

impl FromWorld for HandleMap<ImageKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(
            ImageKey::Ducky,
            asset_server.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        )]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SfxKey {
    ButtonHover,
    ButtonPress,
    Step1,
    Step2,
    Step3,
    Step4,
    Fire,
    Alarm,
    Coin,
    Kitchen,
    ToiletFlush,
    Water,
    Wave,
    Typing,
    Cooking,
}

impl AssetKey for SfxKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SfxKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SfxKey::ButtonHover,
                asset_server.load("audio/sfx/button_hover.ogg"),
            ),
            (
                SfxKey::ButtonPress,
                asset_server.load("audio/sfx/button_press.ogg"),
            ),
            (SfxKey::Step1, asset_server.load("audio/sfx/step1.ogg")),
            (SfxKey::Step2, asset_server.load("audio/sfx/step2.ogg")),
            (SfxKey::Step3, asset_server.load("audio/sfx/step3.ogg")),
            (SfxKey::Step4, asset_server.load("audio/sfx/step4.ogg")),
            (
                SfxKey::Cooking,
                asset_server.load("audio/sfx/dishes_03.ogg"),
            ),
            (SfxKey::Fire, asset_server.load("audio/sfx/fire.ogg")),
            (SfxKey::Alarm, asset_server.load("audio/sfx/alarm.ogg")),
            (
                SfxKey::Typing,
                asset_server.load("audio/sfx/human_typing.ogg"),
            ),
            (
                SfxKey::Coin,
                asset_server.load("audio/sfx/coins_sounds/coin.6.ogg"),
            ),
            (
                SfxKey::Kitchen,
                asset_server.load("audio/sfx/crunch/crunch.2.ogg"),
            ),
            (
                SfxKey::ToiletFlush,
                asset_server.load("audio/sfx/toilet_02.ogg"),
            ),
            (SfxKey::Water, asset_server.load("audio/sfx/bottle.ogg")),
            (SfxKey::Wave, asset_server.load("audio/sfx/wave.ogg")),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Credits,
    Gameplay,
}

impl AssetKey for SoundtrackKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SoundtrackKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SoundtrackKey::Credits,
                asset_server.load("audio/soundtracks/Monkeys Spinning Monkeys.ogg"),
            ),
            (
                SoundtrackKey::Gameplay,
                asset_server.load("audio/soundtracks/Cyberpunk-Moonlight-Sonata-v2.ogg"),
            ),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SceneKey {
    Pc,
    Kitchen,
    WaterTank,
    OxygenTank,
    BadWaterTank,
    HydrogenTank,
    PeeWaterTank,
    MetalTrash,
    Player,
    Hydroponic,
    OxygenGenerator,
    Earth,
    Toilet,
    WaterDispenser,
    WaterCleaner,
}

impl AssetKey for SceneKey {
    type Asset = Scene;
}

impl FromWorld for HandleMap<SceneKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (SceneKey::Pc, asset_server.load("models/pc.glb#Scene0")),
            (
                SceneKey::Kitchen,
                asset_server.load("models/kitchen.glb#Scene0"),
            ),
            (
                SceneKey::WaterTank,
                asset_server.load("models/water_tank.glb#Scene0"),
            ),
            (
                SceneKey::OxygenTank,
                asset_server.load("models/oxygen_tank.glb#Scene0"),
            ),
            (
                SceneKey::BadWaterTank,
                asset_server.load("models/bad_water_tank.glb#Scene0"),
            ),
            (
                SceneKey::PeeWaterTank,
                asset_server.load("models/pee_tank.glb#Scene0"),
            ),
            (
                SceneKey::HydrogenTank,
                asset_server.load("models/hydrogen_tank.glb#Scene0"),
            ),
            (
                SceneKey::MetalTrash,
                asset_server.load("models/metal_trash.glb#Scene0"),
            ),
            (SceneKey::Player, asset_server.load("models/guy.glb#Scene0")),
            (
                SceneKey::Earth,
                asset_server.load("models/earth.glb#Scene0"),
            ),
            (
                SceneKey::Hydroponic,
                asset_server.load("models/hydroponic.glb#Scene0"),
            ),
            (
                SceneKey::OxygenGenerator,
                asset_server.load("models/oxygen_generator.glb#Scene0"),
            ),
            (
                SceneKey::Toilet,
                asset_server.load("models/toilet.glb#Scene0"),
            ),
            (
                SceneKey::WaterDispenser,
                asset_server.load("models/water_dispenser.glb#Scene0"),
            ),
            (
                SceneKey::WaterCleaner,
                asset_server.load("models/water_cleaner.glb#Scene0"),
            ),
        ]
        .into()
    }
}

pub trait AssetKey: Sized {
    type Asset: Asset;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HandleMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for HandleMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> HandleMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
