pub struct VisualNovelPlugin;
use bevy::asset::Error;
use std::path::PathBuf;
use bevy::prelude::*;
use cuentitos_runtime::*;

mod database_asset;
pub use database_asset::DatabaseAsset;




impl Plugin for VisualNovelPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(Cuentitos::default())
      .add_systems(Update, load_database);
  }
}

#[derive(Debug, Default, Resource)]
struct Cuentitos {
  path: PathBuf,
  runtime: Option<Runtime>
}

fn load_database(
  mut cuentitos: ResMut<Cuentitos>,
  asset_server: Res<AssetServer>,
  databases: Res<Assets<DatabaseAsset>>
) {
  if cuentitos.runtime.is_none() {
    let db_handle: Handle<DatabaseAsset> = asset_server.load(cuentitos.path.clone());

    if let Some(database_asset) = databases.get(&db_handle) {
      cuentitos.runtime = Some(Runtime::new(database_asset.database.clone()));
    };
  }
}

#[cfg(test)]
mod test {
  use crate::database_asset::DatabaseAssetLoader;
  use std::str::FromStr;
  use super::*;

  #[test]
  fn load_database_initializes_cuentitos_database() {
    let mut app = App::new();
    app
      .add_plugins((AssetPlugin::default(), TaskPoolPlugin::default()))
      .add_asset::<DatabaseAsset>()
      .init_asset_loader::<DatabaseAssetLoader>()
      .insert_resource(Cuentitos {
        path: PathBuf::from_str("visual-novel-example.db").unwrap(),
        ..default()
      })
      .add_systems(Update, load_database);

    app.update();

    let data = include_bytes!("../assets/visual-novel-example.db");
    let expected_database = Database::from_u8(data).unwrap();
    
    while app.world.resource::<Cuentitos>().runtime.is_none() {
      app.update();
    }
    
    let actual_database = app.world.resource::<Cuentitos>().runtime.clone().unwrap().database;

    assert_eq!(actual_database, expected_database);
  }

  #[test]
  fn load_database_using_async_task() {
    todo!("Load Database using async task");
  }
}