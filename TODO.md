Chunk -> Dice -> Block -> type_id

gl_traits: https://doc.rust-lang.org/book/ch10-02-traits.html#traits-defining-shared-behavior

------------------------------------

# Coordinates

WorldCoord (xyz, for block)

BlockCoord (xyz, county by block, find block in dice)
* baseWorldCoord(DiceCoord, ChunkCoord)

DiceCoord (xyz, count by dice, find dice in chunk)
* baseWorldCoord(ChunkCoord)

ChunkCoord (xyz, world coord count by Chunk)
* baseWorldCoord()

（也许还需要一个fromWorldCoord）

------------------------------------

BlockType
* id, name, other properties
* index in texture array of all sides
  * getters for index for diffuse, 

BlockTypeDefinitions
* id, Name, other properties
* texture name of all sides

BlockTypeRegistry(block_type_definitions, texture_registry)
* `get_by_id(id) -> BlockType`

--------------------------------------

LoadedArea:
 * Array of Chunks, indexed by ChunkCoord
 * prune_and_load()

Chunks
   * Array of Dices
   * Array of DiceMesh

DiceMesh:
  * `new(Vec<Vertex>)`
  * gl_id

ChunkMeshBuilder (BlockTypeRegistry, TextureRegistry) -> Array of DicesMesh (used by chunk)
* build()

Renderer
* camera
* TextureRegistry

TextureRegistry (texture_definition):
* (texture_name, texture_type) -> texture_id
* texture array gl_id;

TextureDefinition
* name
* path_diffuse
* path_specular
* path_glossiness
* path_normal

Texture:
* name
* path_diffuse
* id_diffuse
* path_specular
* id_specular
* path_glossiness
* id_glossiness
* path_normal
* id_normal