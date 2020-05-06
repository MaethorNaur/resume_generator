use printpdf::*;
pub fn rectangle_points(offset_x: Pt, offset_y: Pt, width: Pt, height: Pt) -> Vec<(Point, bool)> {
    let top = offset_y + height;
    let right = offset_x + width;
    let top_left_pt = Point {
        x: offset_x,
        y: top,
    };
    let top_right_pt = Point { x: right, y: top };
    let bottom_right_pt = Point {
        x: right,
        y: offset_y,
    };
    let bottom_left_pt = Point {
        x: offset_x,
        y: offset_y,
    };
    vec![
        (top_left_pt, false),
        (top_right_pt, false),
        (bottom_right_pt, false),
        (bottom_left_pt, false),
    ]
}
