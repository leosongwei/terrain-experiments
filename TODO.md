Chunk -> Dice -> Block -> type_id

BlockType:
  * id
  * name
  * texture_ids

gl_traits: https://doc.rust-lang.org/book/ch10-02-traits.html#traits-defining-shared-behavior

BlockTypeRegistry(block_type_definitions, texture_registry)
* `get_by_id(id) -> BlockType`

BlockTypeDefinitions
* id, Name, other properties
* texture name of all sides

BlockType
* id, name, other properties
* index in texture array of all sides
  * getters for index for diffuse, 

LoadedArea:
 * Array of Chunks

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
* texture_name -> texture_id
* texture array gl_id;

TextureDefinition
* name
* path_diffuse
* path_specular
* path_glossiness
* path_normal

Texture:
* name
* id
* path