#[test]
fn wrap_column() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(30f32),
                    height: taffy::style::Dimension::Points(31f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(30f32),
                    height: taffy::style::Dimension::Points(32f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(30f32),
                    height: taffy::style::Dimension::Points(33f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node3 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(30f32),
                    height: taffy::style::Dimension::Points(34f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    height: taffy::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 31f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 32f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 31f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 33f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 63f32);
    assert_eq!(taffy.layout(node3).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node3).unwrap().size.height, 34f32);
    assert_eq!(taffy.layout(node3).unwrap().location.x, 50f32);
    assert_eq!(taffy.layout(node3).unwrap().location.y, 0f32);
}
