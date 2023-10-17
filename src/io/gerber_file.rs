
enum LayerType {
    COPPER,
    SOLDERMASK,
    SILKCREEN,
}

enum LayerPosition {
    TOP,
    BOTTOM,
}

struct Layer{
    name: String,
    layer_type: LayerType,
    position: LayerPosition
}
struct GerberJob{
    width: f32,
    height: f32,
    board_thickness: f32
}