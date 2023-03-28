use crate::domain::actions::{LinearActionState, NothingActionState};
use crate::domain::actor::SpeedStats;
use crate::domain::map::{DOOR_OPENING_SPEED_IN_UNITS_PER_SECONDS, Map, MapConfiguration, Tile};
use crate::sdl::texture::ResourceRegistry;

pub fn map_loader(registry: &mut ResourceRegistry) -> Map {

    let wal_texture = registry.load_texture(String::from("wall.png"));
    let void_texture = registry.load_texture(String::from("transparency.png"));
    let door_texture = registry.load_texture(String::from("door.png"));
    let glass_texture = registry.load_texture(String::from("glass.png"));

    let mut configuration = MapConfiguration::new(void_texture);
    configuration.add('#', Tile::SOLID(wal_texture));
    configuration.add('D', Tile::DYNAMIC(door_texture, void_texture, || Box::new(LinearActionState::new(SpeedStats::new(DOOR_OPENING_SPEED_IN_UNITS_PER_SECONDS)))));
    configuration.add('G', Tile::DYNAMIC(glass_texture, void_texture, || Box::new(NothingActionState::new())));
    configuration.add(' ', Tile::NOTHING);

    let map = Map::new(
        "\
        ##############\n\
        #      #     #\n\
        #  #   #######\n\
        #  #     #   #\n\
        #  ####  # ###\n\
        #     #      #\n\
        #### ####G#D##\n\
        #            #\n\
        #            #\n\
        #        #   #\n\
        #        D   #\n\
        #        #    \n\
        #        G   #\n\
        #        #   #\n\
        ##############",
        configuration)
        .unwrap();

    map
}
