use bevy::asset::LoadedAsset;
use cuentitos_runtime::*;

use bevy::{
  asset::LoadContext,
  reflect::{TypePath, TypeUuid},
  utils::BoxedFuture
};

use serde::Deserialize;

#[derive(Default, Debug, Deserialize, TypeUuid, TypePath)]
#[uuid = "bf3c7f7a-30a3-11ee-be56-0242ac120002"]
pub struct DatabaseAsset {
  pub database: Database
}

#[derive(Default)]
pub struct DatabaseAssetLoader;

impl bevy::asset::AssetLoader for DatabaseAssetLoader {
  fn load<'a>(
    &'a self,
    bytes: &'a [u8],
    load_context: &'a mut LoadContext,
  ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
    Box::pin(async move {
      let database = match Database::from_u8(bytes) {
        Ok(db) => Ok(db),
        Err(_err) => Err(bevy::asset::Error::msg("Can't load cuentitos database"))
      }?;
      
      load_context.set_default_asset(
        LoadedAsset::new(DatabaseAsset { database })
      );
      Ok(())
    })
  }

  fn extensions(&self) -> &[&str] {
    &["db"]
  }
}

#[cfg(test)]
mod test {
  use bevy::prelude::*;
  use super::*;

  #[derive(Resource, Default)]
  struct State {
    handle: Handle<DatabaseAsset>,
    loaded: bool,
  }

  #[test]
  fn asset_server_understands_cuentitos_dbs() {
    fn setup(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
      state.handle = asset_server.load("visual-novel-example.db");
    }

    fn test_loaded(
      mut state: ResMut<State>,
      databases: Res<Assets<DatabaseAsset>>
    ) {
      let database_asset = databases.get(&state.handle);
      if state.loaded || database_asset.is_none() {
        return;
      }

      let data = include_bytes!("../assets/visual-novel-example.db");
      let expected_database = Database::from_u8(data).unwrap();
      assert_eq!(database_asset.unwrap().database, expected_database);
      state.loaded = true;
    }

    App::new()
      .add_plugins((AssetPlugin::default(), TaskPoolPlugin::default()))
      .init_resource::<State>()
      .add_asset::<DatabaseAsset>()
      .init_asset_loader::<DatabaseAssetLoader>()
      .add_systems(Startup, setup)
      .add_systems(Update, test_loaded)
      .run();
  }
}