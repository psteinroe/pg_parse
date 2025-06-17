//! Tests nested packages without a root package.

include!(concat!(env!("OUT_DIR"), "/no_root_packages/__.default.rs"));

pub mod gizmo {
    include!(concat!(env!("OUT_DIR"), "/no_root_packages/gizmo.rs"));
}

pub mod widget {
    include!(concat!(env!("OUT_DIR"), "/no_root_packages/widget.rs"));
    pub mod factory {
        include!(concat!(
            env!("OUT_DIR"),
            "/no_root_packages/widget.factory.rs"
        ));
    }
}

pub mod generated_include {
    include!(concat!(env!("OUT_DIR"), "/no_root_packages/__.include.rs"));
}

#[test]
fn test() {
    use prost::Message;

    let mut widget_factory = widget::factory::WidgetFactory::default();
    assert_eq!(0, widget_factory.encoded_len());

    widget_factory.inner = Some(widget::factory::widget_factory::Inner {});
    assert_eq!(2, widget_factory.encoded_len());

    widget_factory.root = Some(Root {});
    assert_eq!(4, widget_factory.encoded_len());

    widget_factory.root_inner = Some(root::Inner {});
    assert_eq!(6, widget_factory.encoded_len());

    widget_factory.widget = Some(widget::Widget {});
    assert_eq!(8, widget_factory.encoded_len());

    widget_factory.widget_inner = Some(widget::widget::Inner {});
    assert_eq!(10, widget_factory.encoded_len());

    widget_factory.gizmo = Some(gizmo::Gizmo {});
    assert_eq!(12, widget_factory.encoded_len());

    widget_factory.gizmo_inner = Some(gizmo::gizmo::Inner {});
    assert_eq!(14, widget_factory.encoded_len());
}

#[test]
fn generated_include() {
    use prost::Message;

    let mut widget_factory = generated_include::widget::factory::WidgetFactory::default();
    assert_eq!(0, widget_factory.encoded_len());

    widget_factory.inner = Some(generated_include::widget::factory::widget_factory::Inner {});
    assert_eq!(2, widget_factory.encoded_len());

    widget_factory.root = Some(generated_include::Root {});
    assert_eq!(4, widget_factory.encoded_len());

    widget_factory.root_inner = Some(generated_include::root::Inner {});
    assert_eq!(6, widget_factory.encoded_len());

    widget_factory.widget = Some(generated_include::widget::Widget {});
    assert_eq!(8, widget_factory.encoded_len());

    widget_factory.widget_inner = Some(generated_include::widget::widget::Inner {});
    assert_eq!(10, widget_factory.encoded_len());

    widget_factory.gizmo = Some(generated_include::gizmo::Gizmo {});
    assert_eq!(12, widget_factory.encoded_len());

    widget_factory.gizmo_inner = Some(generated_include::gizmo::gizmo::Inner {});
    assert_eq!(14, widget_factory.encoded_len());
}
